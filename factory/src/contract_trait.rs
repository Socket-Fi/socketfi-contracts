use socketfi_shared::ContractError;
use soroban_sdk::{Address, BytesN, Env, String, Vec};

/// Public interface for the factory contract.
///
/// This trait defines the external contract surface for:
/// - initialization
/// - wallet creation
/// - configuration reads and updates
/// - upgrade governance actions
///
/// Audit note:
/// - keep this trait aligned with the implementation order in `contract.rs`
///   to improve readability and reduce maintenance mistakes
pub trait FactoryTrait {
    // ---------------------------------------------------------------------
    // Initialization
    // ---------------------------------------------------------------------

    /// Initializes the factory contract and core dependencies.
    fn __constructor(
        e: Env,
        admin: Address,
        registry: Address,
        fee_manager: Address,
        wasm: BytesN<32>,
    ) -> Result<(), ContractError>;

    // ---------------------------------------------------------------------
    // Wallet creation
    // ---------------------------------------------------------------------

    /// Creates a new wallet instance.
    fn create_wallet(
        e: Env,
        passkey: BytesN<77>,
        bls_keys: Vec<BytesN<96>>,
    ) -> Result<Address, ContractError>;

    // ---------------------------------------------------------------------
    // Read-only getters
    // ---------------------------------------------------------------------

    /// Returns the currently approved wallet version hash.
    fn get_wallet_version(e: Env) -> Result<BytesN<32>, ContractError>;

    /// Returns the current admin address.
    fn get_admin(e: Env) -> Result<Address, ContractError>;

    /// Returns the configured registry contract address.
    fn get_registry(e: Env) -> Result<Address, ContractError>;

    /// Returns the configured fee manager contract address.
    fn get_fee_manager(e: Env) -> Result<Address, ContractError>;

    // ---------------------------------------------------------------------
    // Admin configuration updates
    // ---------------------------------------------------------------------

    /// Updates the current admin address.
    fn update_admin(e: Env, new_admin: Address) -> Result<(), ContractError>;

    /// Updates the registry contract address.
    fn update_registry(e: Env, registry: Address) -> Result<(), ContractError>;

    /// Updates the fee manager contract address.
    fn update_fee_manager(e: Env, fee_manager: Address) -> Result<(), ContractError>;

    // ---------------------------------------------------------------------
    // Upgrade governance
    // ---------------------------------------------------------------------

    /// Executes a passed upgrade proposal.
    fn apply_upgrade(e: Env) -> Result<BytesN<32>, ContractError>;

    /// Creates a new upgrade proposal.
    fn propose_upgrade(
        e: Env,
        proposal_type: String,
        new_wasm_hash: BytesN<32>,
    ) -> Result<(), ContractError>;

    /// Adds a voter to the upgrade governance set.
    fn add_voter(e: Env, voter: Address) -> Result<(), ContractError>;

    /// Casts a vote for an active upgrade proposal.
    fn cast_vote(e: Env, voter: Address, wasm_hash: BytesN<32>) -> Result<(), ContractError>;

    /// Cancels the current active proposal.
    fn cancel_proposal(e: Env) -> Result<(), ContractError>;
}
