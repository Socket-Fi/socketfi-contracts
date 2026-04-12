#![no_std]

mod storage;
pub mod voters;

use crate::storage::{
    clear_pending_upgrade_state, get_future_wasm, get_upgrade_voting_deadline, write_future_wasm,
    write_upgrade_voting_deadline, write_wallet_version, DataKey,
};
use crate::voters::{read_has_upgrade_passed, read_is_voter};
use socketfi_shared::{
    constants::UPGRADE_VOTING_DURATION, events, types::UpgradeType, ContractError,
};
use soroban_sdk::{Address, BytesN, Env, Map, String};
use storage::read_wallet_version;
use voters::write_add_voter;

/// Initializes the wallet version.
///
/// This function is intended to be called **once during contract deployment**
/// (e.g. inside the wallet constructor or initialization function).
///
/// Behavior:
/// - sets the initial wallet version hash
/// - prevents re-initialization if already set
///
/// Requirements:
/// - must not have been initialized before
///
/// Security notes:
/// - this should only be callable during contract initialization
///   (e.g. from `__constructor` or a protected init function)
/// - re-initialization is blocked to prevent version hijacking
///
/// Design notes:
/// - `wallet_version` typically represents the approved wallet WASM hash
///   or a canonical version identifier used across upgrades
/// - subsequent updates to the wallet version should go through the
///   governance flow (`create_upgrade_proposal` + `execute_upgrade`)
///
/// Errors:
/// - `AlreadyInitialized` if the wallet version has already been set
pub fn init_wallet_version(e: &Env, wallet_version: &BytesN<32>) -> Result<(), ContractError> {
    if e.storage().persistent().has(&DataKey::WalletVersion) {
        return Err(ContractError::AlreadyInitialized);
    }

    write_wallet_version(e, wallet_version)?;
    Ok(())
}

/// Creates a new upgrade proposal.
///
/// Flow:
/// 1. ensure no other proposal is active
/// 2. store voting deadline
/// 3. store target WASM + proposal type
///
/// Security note:
/// - caller should enforce proposer authorization before calling this helper.
pub fn create_upgrade_proposal(
    e: &Env,
    proposal_type: String,
    wasm_hash: &BytesN<32>,
) -> Result<(), ContractError> {
    if get_upgrade_voting_deadline(e) != 0 {
        return Err(ContractError::AnotherUpgradePending);
    }

    let deadline = e.ledger().timestamp() + UPGRADE_VOTING_DURATION;
    write_upgrade_voting_deadline(e, &deadline);
    write_future_wasm(e, proposal_type, wasm_hash)?;

    events::UpgradeProposalEvent {
        wasm: wasm_hash.clone(),
        voting_deadline: deadline,
    }
    .publish(&e);

    Ok(())
}

/// Casts a vote for the current active proposal.
///
/// Requirements:
/// - there must be an active proposal
/// - voting must still be open
/// - `voter` must be in the approved voter list
/// - `voter` must authorize the call
/// - `wasm_hash` must match the active proposal
/// - voter may only vote once
pub fn write_cast_vote(
    e: &Env,
    voter: &Address,
    wasm_hash: &BytesN<32>,
) -> Result<(), ContractError> {
    let deadline = get_upgrade_voting_deadline(e);

    if deadline == 0 {
        return Err(ContractError::NoPendingUpgradeAction);
    }

    if e.ledger().timestamp() > deadline {
        return Err(ContractError::VotingClosed);
    }

    if !read_is_voter(e, voter.clone()) {
        return Err(ContractError::NotInVotersList);
    }

    let (_, future_wasm_hash) = get_future_wasm(e)?;
    if future_wasm_hash != *wasm_hash {
        return Err(ContractError::InvalidUpgradeHash);
    }

    let key = DataKey::VotedList;
    let mut voted: Map<Address, ()> = e.storage().persistent().get(&key).unwrap_or(Map::new(e));

    if voted.contains_key(voter.clone()) {
        return Err(ContractError::AlreadyVoted);
    }

    voted.set(voter.clone(), ());
    e.storage().persistent().set(&key, &voted);

    events::VoteEvent {
        wasm: wasm_hash.clone(),
        voter: voter.clone(),
    }
    .publish(&e);

    Ok(())
}

/// Finalizes the currently active proposal after voting ends.
///
/// Behavior depends on proposal type:
/// - `Upgrade`      -> upgrades current contract WASM
/// - `WalletVersion` -> records approved wallet version hash
///
/// Security notes:
/// - execution is only allowed after deadline
/// - proposal must have passed threshold
/// - pending proposal state is cleared on success
///
/// Important:
/// - for `Upgrade`, state is cleared before `update_current_contract_wasm`
///   to avoid leaving stale proposal state behind.
/// - for `WalletVersion`, the version write happens first because it may fail.
pub fn execute_upgrade(e: &Env) -> Result<BytesN<32>, ContractError> {
    let deadline = get_upgrade_voting_deadline(e);

    if deadline == 0 {
        return Err(ContractError::NoPendingUpgradeAction);
    }

    if e.ledger().timestamp() < deadline {
        return Err(ContractError::VotingStillOngoing);
    }

    let (proposal_type, new_wasm_hash) = get_future_wasm(e)?;
    let (_, has_passed) = read_has_upgrade_passed(e)?;

    if !has_passed {
        return Err(ContractError::DidNotPass);
    }

    match proposal_type {
        UpgradeType::Upgrade => {
            clear_pending_upgrade_state(e);
            e.deployer()
                .update_current_contract_wasm(new_wasm_hash.clone());
            events::ContractUpgradeEvent {
                wasm: new_wasm_hash.clone(),
            }
            .publish(&e);
        }
        UpgradeType::WalletVersion => {
            write_wallet_version(e, &new_wasm_hash)?;
            clear_pending_upgrade_state(e);
            events::WalletVersionUpgradeEvent {
                wasm: new_wasm_hash.clone(),
            }
            .publish(&e);
        }
    }

    Ok(new_wasm_hash)
}

/// Cancels the currently active proposal and clears all pending proposal state.
///
/// Security note:
/// - caller should enforce authorization before calling this helper.
pub fn cancel_upgrade_proposal(e: &Env) -> Result<(), ContractError> {
    let (_, wasm) = get_future_wasm(e)?;
    clear_pending_upgrade_state(e);
    events::UpgradeCancelledEvent { wasm: wasm.clone() }.publish(&e);
    Ok(())
}

/// Returns `(vote_count, has_passed)` for the current active proposal.
pub fn get_upgrade_votes(e: &Env) -> Result<(u32, bool), ContractError> {
    read_has_upgrade_passed(e)
}
/// Returns `(vote_count, has_passed)` for the current active proposal.
pub fn get_wallet_version(e: &Env) -> Result<BytesN<32>, ContractError> {
    read_wallet_version(e)
}
/// Add new Voter `(vote_count, has_passed)` for the current active proposal.
pub fn upgrade_add_voter(e: &Env, voter: &Address) -> Result<(), ContractError> {
    write_add_voter(e, voter)?;
    Ok(())
}
