use socketfi_shared::utils::bump_instance;
use socketfi_shared::{types::UpgradeType, ContractError};
use soroban_sdk::{contracttype, BytesN, Env, String};

/// Storage keys used by the upgrade governance module.
///
/// Design notes:
/// - Only one proposal can be active at a time.
/// - `FutureWASM` and `ProposalType` always belong to the same active proposal.
/// - `VotedList` is global because only one proposal is allowed at a time.
/// - `NewWalletVersion` stores the approved wallet implementation hash when the
///   proposal type is `WalletVersion`.
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// UNIX timestamp after which the pending proposal may be executed.
    UpgradeVotingDeadline,

    /// WASM hash under vote.
    FutureWASM,

    /// Approved voter set.
    VotersList,

    /// Addresses that have voted for the current active proposal.
    VotedList,

    /// Latest approved wallet version hash.
    WalletVersion,

    /// Type of the currently active proposal.
    ProposalType,
}

/// Returns the active voting deadline.
///
/// Returns `0` when there is no active proposal.
///
/// Security note:
/// - A deadline of `0` is treated as "no pending proposal".
pub fn get_upgrade_voting_deadline(e: &Env) -> u64 {
    // bump_instance(e);
    e.storage()
        .instance()
        .get(&DataKey::UpgradeVotingDeadline)
        .unwrap_or(0)
}

/// Writes the active voting deadline.
pub fn write_upgrade_voting_deadline(e: &Env, value: &u64) {
    // bump_instance(e);
    e.storage()
        .instance()
        .set(&DataKey::UpgradeVotingDeadline, value);
}

/// Returns the pending proposal payload as `(proposal_type, wasm_hash)`.
///
/// Errors:
/// - `UpgradeWasmNotFound` if no pending WASM hash exists
/// - `UpgradeTypeNotFound` if proposal type was not stored
///
/// Invariant:
/// - `FutureWASM` and `ProposalType` must always be set together.
pub fn get_future_wasm(e: &Env) -> Result<(UpgradeType, BytesN<32>), ContractError> {
    // bump_instance(e);

    let wasm = e
        .storage()
        .instance()
        .get(&DataKey::FutureWASM)
        .ok_or(ContractError::UpgradeWasmNotFound)?;

    let proposal_type = e
        .storage()
        .instance()
        .get(&DataKey::ProposalType)
        .ok_or(ContractError::UpgradeTypeNotFound)?;

    Ok((proposal_type, wasm))
}

/// Stores a new pending proposal.
///
/// `proposal_type` is provided as a string and converted into `UpgradeType`.
///
/// Errors:
/// - returns parsing/validation error if proposal type is unsupported
///
/// Security note:
/// - caller should enforce authorization before calling this helper.
pub fn write_future_wasm(
    e: &Env,
    proposal_type: String,
    wasm: &BytesN<32>,
) -> Result<(), ContractError> {
    bump_instance(e);

    let proposal_type = UpgradeType::upgrade_type(proposal_type)?;

    e.storage().instance().set(&DataKey::FutureWASM, wasm);
    e.storage()
        .instance()
        .set(&DataKey::ProposalType, &proposal_type);

    Ok(())
}

/// Stores the approved wallet implementation hash.
///
/// Used only when a `WalletVersion` proposal passes.
pub fn write_wallet_version(e: &Env, wasm_hash: &BytesN<32>) -> Result<(), ContractError> {
    bump_instance(e);
    e.storage()
        .instance()
        .set(&DataKey::WalletVersion, wasm_hash);
    Ok(())
}
/// Returns the approved wallet implementation hash.
///
/// Used for wallet creation and upgrades.
pub fn read_wallet_version(e: &Env) -> Result<BytesN<32>, ContractError> {
    let wasm = e
        .storage()
        .instance()
        .get(&DataKey::WalletVersion)
        .ok_or(ContractError::WalletWasmNotFound)?;
    Ok(wasm)
}

/// Clears all state associated with the currently active proposal.
///
/// This must be called:
/// - after successful execution
/// - after successful wallet version confirmation
/// - on cancellation
///
/// Security note:
/// - because the design allows only one active proposal at a time, `VotedList`
///   is cleared globally.
pub fn clear_pending_upgrade_state(e: &Env) {
    // bump_instance(e);

    e.storage()
        .instance()
        .remove(&DataKey::UpgradeVotingDeadline);
    e.storage().instance().remove(&DataKey::FutureWASM);
    e.storage().instance().remove(&DataKey::ProposalType);
    e.storage().persistent().remove(&DataKey::VotedList);
}
