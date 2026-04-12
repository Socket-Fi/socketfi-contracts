use crate::{
    access::{
        authenticate_admin, has_admin, read_admin, read_fee_manager, read_registry, write_admin,
        write_fee_manager, write_registry,
    },
    contract_trait::FactoryTrait,
    wallet_factory::write_create_wallet,
};
use socketfi_shared::{events, ContractError};
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};
use upgrade::{
    cancel_upgrade_proposal, create_upgrade_proposal, execute_upgrade,
    get_wallet_version as read_wallet_version, init_wallet_version, upgrade_add_voter,
    write_cast_vote,
};

/// Factory contract responsible for:
/// - creating wallet instances
/// - storing core dependency contract addresses
/// - exposing upgrade governance entrypoints
///
/// Security model:
/// - admin controls system configuration and governance administration
/// - approved voters participate in upgrade voting
/// - wallet creation is public unless restricted in downstream logic
#[contract]
pub struct FactoryContract;

#[contractimpl]
impl FactoryTrait for FactoryContract {
    // ---------------------------------------------------------------------
    // Initialization
    // ---------------------------------------------------------------------

    /// Initializes the factory contract.
    ///
    /// Sets:
    /// - admin address
    /// - registry contract address
    /// - fee manager contract address
    /// - initial wallet version hash
    ///
    /// Also adds the initial admin as an upgrade voter.
    ///
    /// Security:
    /// - must only be executed once
    /// - re-initialization is blocked by `has_admin`
    fn __constructor(
        e: Env,
        admin: Address,
        registry: Address,
        fee_manager: Address,
        wasm: BytesN<32>,
    ) -> Result<(), ContractError> {
        if has_admin(&e) {
            return Err(ContractError::AlreadyInitialized);
        }

        write_admin(&e, &admin);
        write_registry(&e, &registry);
        write_fee_manager(&e, &fee_manager);

        // Store the initial approved wallet version.
        init_wallet_version(&e, &wasm)?;

        // Bootstrap governance by allowing the initial admin to vote.
        upgrade_add_voter(&e, &admin)?;

        Ok(())
    }

    // ---------------------------------------------------------------------
    // Wallet creation
    // ---------------------------------------------------------------------

    /// Creates a new wallet instance using the currently configured wallet logic.
    ///
    /// Parameters:
    /// - `passkey`: wallet passkey material / identifier
    /// - `bls_keys`: additional BLS public keys to attach to the wallet
    ///
    /// Emits:
    /// - `WalletCreationEvent`
    ///
    /// Audit note:
    /// - wallet creation is currently permissionless from this contract layer
    /// - any constraints on who may create wallets must be enforced inside
    ///   `write_create_wallet` or downstream wallet initialization logic
    fn create_wallet(
        e: Env,
        passkey: BytesN<77>,
        bls_keys: Vec<BytesN<96>>,
    ) -> Result<Address, ContractError> {
        let wallet_address = write_create_wallet(&e, &passkey, bls_keys)?;

        events::WalletCreationEvent {
            wallet: wallet_address.clone(),
        }
        .publish(&e);

        Ok(wallet_address)
    }

    // ---------------------------------------------------------------------
    // Read-only getters
    // ---------------------------------------------------------------------

    /// Returns the currently approved wallet version hash.
    fn get_wallet_version(e: Env) -> Result<BytesN<32>, ContractError> {
        read_wallet_version(&e)
    }

    /// Returns the current admin address.
    fn get_admin(e: Env) -> Result<Address, ContractError> {
        read_admin(&e)
    }

    /// Returns the configured identity/registry contract address.
    fn get_registry(e: Env) -> Result<Address, ContractError> {
        read_registry(&e)
    }

    /// Returns the configured fee manager contract address.
    fn get_fee_manager(e: Env) -> Result<Address, ContractError> {
        read_fee_manager(&e)
    }

    // ---------------------------------------------------------------------
    // Admin configuration updates
    // ---------------------------------------------------------------------

    /// Updates the admin address.
    ///
    /// Security:
    /// - current admin authorization required
    ///
    /// Emits:
    /// - `UpdateAdminEvent`
    ///
    /// Audit note:
    /// - consider whether rotating admin should also rotate governance voter
    ///   membership automatically, depending on intended governance model
    fn update_admin(e: Env, new_admin: Address) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        write_admin(&e, &new_admin);

        events::UpdateAdminEvent {
            value: new_admin.clone(),
        }
        .publish(&e);

        Ok(())
    }

    /// Updates the registry contract address.
    ///
    /// Security:
    /// - admin only
    ///
    /// Emits:
    /// - `UpdateRegistryEvent`
    fn update_registry(e: Env, registry: Address) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        write_registry(&e, &registry);

        events::UpdateRegistryEvent {
            value: registry.clone(),
        }
        .publish(&e);

        Ok(())
    }

    /// Updates the fee manager contract address.
    ///
    /// Security:
    /// - admin only
    ///
    /// Emits:
    /// - `UpdateFeeManagerEvent`
    fn update_fee_manager(e: Env, fee_manager: Address) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        write_fee_manager(&e, &fee_manager);

        events::UpdateFeeManagerEvent {
            value: fee_manager.clone(),
        }
        .publish(&e);

        Ok(())
    }

    // ---------------------------------------------------------------------
    // Upgrade governance
    // ---------------------------------------------------------------------

    /// Applies a completed upgrade proposal after voting has ended and passed.
    ///
    /// Security:
    /// - admin authorization required to trigger execution
    ///
    /// Returns:
    /// - the approved WASM hash that was applied or recorded
    ///
    /// Audit note:
    /// - actual voting/deadline/pass checks are enforced in `execute_upgrade`
    fn apply_upgrade(e: Env) -> Result<BytesN<32>, ContractError> {
        authenticate_admin(&e)?;
        execute_upgrade(&e)
    }

    /// Creates a new upgrade proposal.
    ///
    /// Parameters:
    /// - `proposal_type`: proposal category, e.g. contract upgrade or wallet version update
    /// - `new_wasm_hash`: target WASM hash under consideration
    ///
    /// Security:
    /// - admin only
    ///
    /// Audit note:
    /// - `proposal_type` is a `String`, which is flexible but weaker than an enum.
    ///   If possible, prefer a strongly typed enum for safer validation.
    fn propose_upgrade(
        e: Env,
        proposal_type: String,
        new_wasm_hash: BytesN<32>,
    ) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        create_upgrade_proposal(&e, proposal_type, &new_wasm_hash)?;
        Ok(())
    }

    /// Adds a new authorized voter for upgrade governance.
    ///
    /// Security:
    /// - admin only
    ///
    /// Emits:
    /// - `AddVoterEvent`
    ///
    /// Audit note:
    /// - ensure downstream voter storage prevents duplicate voter insertion
    fn add_voter(e: Env, voter: Address) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        upgrade_add_voter(&e, &voter)?;

        events::AddVoterEvent {
            value: voter.clone(),
        }
        .publish(&e);

        Ok(())
    }

    /// Casts a vote for the currently active upgrade proposal.
    ///
    /// Parameters:
    /// - `voter`: voter address casting the vote
    /// - `wasm_hash`: the proposal hash the voter is approving
    ///
    /// Security:
    /// - voter authorization required
    ///
    /// Audit note:
    /// - if `write_cast_vote` already calls `require_auth`, the explicit auth
    ///   check here is redundant but harmless
    /// - only approved voters should succeed; that validation is expected
    ///   inside `write_cast_vote`
    fn cast_vote(e: Env, voter: Address, wasm_hash: BytesN<32>) -> Result<(), ContractError> {
        voter.require_auth();
        write_cast_vote(&e, &voter, &wasm_hash)?;
        Ok(())
    }

    /// Cancels the currently active upgrade proposal.
    ///
    /// Security:
    /// - admin only
    ///
    /// Audit note:
    /// - helper is expected to clear all pending proposal state safely
    /// - consider emitting a cancel event in the upgrade module if not already done
    fn cancel_proposal(e: Env) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        cancel_upgrade_proposal(&e)?;
        Ok(())
    }
}
