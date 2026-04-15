use soroban_sdk::{Address, BytesN, Env, String, Vec};

use socketfi_shared::{types::ValidatorSignature, ContractError};

pub trait RegistryTrait {
    // ---------------------------------------------------------------------
    // Initialization
    // ---------------------------------------------------------------------

    fn __constructor(
        e: Env,
        admin: Address,
        factory: Address,
        social_payments: Address,
    ) -> Result<(), ContractError>;

    // ---------------------------------------------------------------------
    // Identity Core
    // ---------------------------------------------------------------------

    fn verify_identity_binding(
        e: Env,
        wallet: Address,
        user_id: String,
        platform_str: String,
        signatures: Vec<ValidatorSignature>,
    ) -> Result<(), ContractError>;

    fn set_passkey_wallet_map(
        e: Env,
        passkey: BytesN<77>,
        wallet: Address,
    ) -> Result<(), ContractError>;

    // ---------------------------------------------------------------------
    // Validator Management
    // ---------------------------------------------------------------------

    fn add_validator(e: Env, validator: BytesN<32>) -> Result<(), ContractError>;

    fn remove_validator(e: Env, validator: BytesN<32>) -> Result<(), ContractError>;

    fn get_validators(e: Env) -> Result<Vec<BytesN<32>>, ContractError>;

    // ---------------------------------------------------------------------
    // Read APIs
    // ---------------------------------------------------------------------

    fn get_wallet_by_userid(
        e: Env,
        platform: String,
        user_id: String,
    ) -> Result<Option<Address>, ContractError>;

    fn get_wallet_by_passkey(e: Env, passkey: BytesN<77>)
        -> Result<Option<Address>, ContractError>;

    fn get_factory(e: Env) -> Result<Address, ContractError>;

    fn get_social_payments(e: Env) -> Result<Address, ContractError>;

    // ---------------------------------------------------------------------
    // Admin / Config
    // ---------------------------------------------------------------------

    fn set_admin(e: Env, new_admin: Address) -> Result<(), ContractError>;

    // ---------------------------------------------------------------------
    // Upgrade Governance
    // ---------------------------------------------------------------------

    fn apply_upgrade(e: Env) -> Result<BytesN<32>, ContractError>;

    fn propose_upgrade(
        e: Env,
        proposal_type: String,
        new_wasm_hash: BytesN<32>,
    ) -> Result<(), ContractError>;

    fn add_voter(e: Env, voter: Address) -> Result<(), ContractError>;

    fn cast_vote(e: Env, voter: Address, wasm_hash: BytesN<32>) -> Result<(), ContractError>;

    fn cancel_proposal(e: Env) -> Result<(), ContractError>;

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) -> Result<(), ContractError>;
}
