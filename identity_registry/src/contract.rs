use soroban_sdk::{
    contract, contractimpl, xdr::ToXdr, Address, Bytes, BytesN, Env, Map, String, Vec,
};

use crate::{
    access::{
        authenticate_admin, has_admin, read_escrow, read_factory, write_admin, write_escrow,
        write_factory,
    },
    contract_trait::RegistryTrait,
    registry::{
        read_passkey_wallet_map, read_userid_wallet_map, write_passkey_wallet_map,
        write_userid_wallet_map,
    },
    validators::{
        read_is_validator, read_threshold, read_validators, write_add_validator,
        write_remove_validator,
    },
};
use socketfi_shared::{
    events,
    types::{SocialPlatform, ValidatorSignature},
    utils::validate_userid,
    ContractError,
};
use upgrade::{
    cancel_upgrade_proposal, create_upgrade_proposal, execute_upgrade, upgrade_add_voter,
    write_cast_vote,
};

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryTrait for RegistryContract {
    // ---------------------------------------------------------------------
    // Initialization
    // ---------------------------------------------------------------------

    /// Initializes the registry contract.
    ///
    /// Stores:
    /// - `admin`: privileged controller for registry configuration
    /// - `factory`: trusted contract allowed to register passkey mappings
    /// - `escrow`: linked escrow / pending-payments contract
    ///
    /// Security:
    /// - Single-use initialization guarded by `has_admin`
    ///
    /// Audit notes:
    /// - Assumes constructor execution is atomic
    /// - If instance TTL is not maintained, config values may expire
    fn __constructor(
        e: Env,
        admin: Address,
        factory: Address,
        escrow: Address,
    ) -> Result<(), ContractError> {
        if has_admin(&e) {
            return Err(ContractError::AlreadyInitialized);
        }

        write_admin(&e, &admin);
        write_factory(&e, &factory);
        write_escrow(&e, &escrow);

        Ok(())
    }

    // ---------------------------------------------------------------------
    // Identity Core
    // ---------------------------------------------------------------------

    /// Verifies validator-approved identity binding and stores the mapping.
    ///
    /// Authorization:
    /// - Wallet owner must authorize
    ///
    /// Signed payload fields:
    /// - action name: `"verify_identity_binding"`
    /// - current contract address
    /// - wallet address
    /// - canonical platform string
    /// - validated `user_id`
    ///
    /// Security properties:
    /// - Contract address provides domain separation
    /// - Validator membership is checked on-chain
    /// - Duplicate validator signatures are rejected
    /// - Platform is canonicalized before signing
    /// - `user_id` is validated before signing
    ///
    /// Audit notes:
    /// - Full XDR encoding is used for all signed fields to avoid ambiguous encoding
    /// - Replay protection is not currently included
    /// - Storage-layer rebinding prevention acts as the current one-time-use guard
    /// - Consider emitting an identity-bound event for indexers / escrow release flows
    fn verify_identity_binding(
        e: Env,
        wallet: Address,
        user_id: String,
        platform_str: String,
        signatures: Vec<ValidatorSignature>,
    ) -> Result<(), ContractError> {
        wallet.require_auth();

        let platform = SocialPlatform::is_platform_supported(platform_str)?;
        validate_userid(user_id.clone())?;

        let threshold = read_threshold(&e);
        if threshold == 0 {
            return Err(ContractError::InvalidThreshold);
        }

        if signatures.len() as u32 != threshold {
            return Err(ContractError::IncorrectNumberOfSignatures);
        }

        let mut seen = Map::<BytesN<32>, bool>::new(&e);

        for s in signatures.iter() {
            let validator = s.validator.clone();

            if !read_is_validator(&e, validator.clone()) {
                return Err(ContractError::NotValidator);
            }

            if seen.get(validator.clone()).unwrap_or(false) {
                return Err(ContractError::DuplicateValidator);
            }

            seen.set(validator, true);
        }

        let mut message = Bytes::new(&e);
        message.append(&String::from_str(&e, "verify_identity_binding").to_xdr(&e));
        message.append(&e.current_contract_address().to_xdr(&e));
        message.append(&wallet.clone().to_xdr(&e));
        message.append(&String::from_str(&e, platform.as_str()).to_xdr(&e));
        message.append(&user_id.clone().to_xdr(&e));

        for s in signatures.iter() {
            e.crypto()
                .ed25519_verify(&s.validator, &message, &s.signature);
        }

        write_userid_wallet_map(&e, String::from_str(&e, platform.as_str()), user_id, wallet)?;

        // Recommended:
        // emit IdentityBound event here

        Ok(())
    }

    /// Registers a passkey -> wallet mapping.
    ///
    /// Authorization:
    /// - Factory only
    ///
    /// Audit notes:
    /// - Trust model assumes the configured factory is the sole authorized creator
    ///   of passkey mappings
    /// - Does not require direct wallet authorization
    /// - Rebinding protection is enforced in storage write logic
    fn set_passkey_wallet_map(
        e: Env,
        passkey: BytesN<77>,
        wallet: Address,
    ) -> Result<(), ContractError> {
        read_factory(&e)?.require_auth();
        write_passkey_wallet_map(&e, passkey, wallet)
    }

    // ---------------------------------------------------------------------
    // Validator Management
    // ---------------------------------------------------------------------

    /// Adds a validator public key.
    ///
    /// Authorization:
    /// - Admin only
    ///
    /// Audit notes:
    /// - Validator membership directly affects identity binding authorization
    fn add_validator(e: Env, validator: BytesN<32>) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        write_add_validator(&e, validator)
    }

    /// Removes a validator public key.
    ///
    /// Authorization:
    /// - Admin only
    ///
    /// Audit notes:
    /// - Removing validators changes the effective signer set for future bindings
    fn remove_validator(e: Env, validator: BytesN<32>) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        write_remove_validator(&e, validator)
    }

    /// Returns the current validator set.
    fn get_validators(e: Env) -> Result<Vec<BytesN<32>>, ContractError> {
        read_validators(&e)
    }

    // ---------------------------------------------------------------------
    // Read APIs
    // ---------------------------------------------------------------------

    /// Returns wallet bound to a `(platform, user_id)` pair, if present.
    ///
    /// Audit notes:
    /// - Lookup correctness depends on the same canonicalization rules used during write
    fn get_wallet_by_userid(
        e: Env,
        platform: String,
        user_id: String,
    ) -> Result<Option<Address>, ContractError> {
        read_userid_wallet_map(&e, platform, user_id)
    }

    /// Returns wallet bound to a passkey, if present.
    fn get_wallet_by_passkey(
        e: Env,
        passkey: BytesN<77>,
    ) -> Result<Option<Address>, ContractError> {
        read_passkey_wallet_map(&e, passkey)
    }

    /// Returns configured factory contract address.
    fn get_factory(e: Env) -> Result<Address, ContractError> {
        read_factory(&e)
    }

    /// Returns configured escrow contract address.
    fn get_escrow(e: Env) -> Result<Address, ContractError> {
        read_escrow(&e)
    }

    // ---------------------------------------------------------------------
    // Admin / Config
    // ---------------------------------------------------------------------

    /// Updates the admin address.
    ///
    /// Authorization:
    /// - Current admin only
    ///
    /// Audit notes:
    /// - Consider emitting an event for admin rotation
    fn set_admin(e: Env, new_admin: Address) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        write_admin(&e, &new_admin);
        Ok(())
    }

    // ---------------------------------------------------------------------
    // Upgrade Governance
    // ---------------------------------------------------------------------

    /// Applies an approved upgrade proposal.
    ///
    /// Authorization:
    /// - Admin only
    ///
    /// Audit notes:
    /// - Proposal voting / pass / deadline checks are enforced in `execute_upgrade`
    fn apply_upgrade(e: Env) -> Result<BytesN<32>, ContractError> {
        authenticate_admin(&e)?;
        execute_upgrade(&e)
    }

    /// Creates a new upgrade proposal.
    ///
    /// Authorization:
    /// - Admin only
    fn propose_upgrade(
        e: Env,
        proposal_type: String,
        new_wasm_hash: BytesN<32>,
    ) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        create_upgrade_proposal(&e, proposal_type, &new_wasm_hash)?;
        Ok(())
    }

    /// Adds a governance voter.
    ///
    /// Authorization:
    /// - Admin only
    ///
    /// Audit notes:
    /// - Event emission improves governance observability
    fn add_voter(e: Env, voter: Address) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        upgrade_add_voter(&e, &voter)?;

        events::AddVoterEvent {
            value: voter.clone(),
        }
        .publish(&e);

        Ok(())
    }

    /// Casts an upgrade vote.
    ///
    /// Authorization:
    /// - Voter must authorize
    fn cast_vote(e: Env, voter: Address, wasm_hash: BytesN<32>) -> Result<(), ContractError> {
        voter.require_auth();
        write_cast_vote(&e, &voter, &wasm_hash)?;
        Ok(())
    }

    /// Cancels the active upgrade proposal.
    ///
    /// Authorization:
    /// - Admin only
    fn cancel_proposal(e: Env) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        cancel_upgrade_proposal(&e)?;
        Ok(())
    }

    /// Performs a direct contract WASM upgrade.
    ///
    /// Authorization:
    /// - Admin only
    ///
    /// Audit notes:
    /// - Highly privileged operation
    /// - Should remain tightly controlled
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        e.deployer().update_current_contract_wasm(new_wasm_hash);
        Ok(())
    }
}
