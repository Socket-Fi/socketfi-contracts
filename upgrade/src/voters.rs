use crate::storage::{get_upgrade_voting_deadline, DataKey};
use socketfi_shared::{constants::VOTING_THRESHOLD, ContractError};
use soroban_sdk::{Address, Env, Map, Vec};

/// Returns true if `voter` is in the approved voter set.
pub fn read_is_voter(env: &Env, voter: Address) -> bool {
    let key = DataKey::VotersList;
    let voters: Map<Address, ()> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(Map::new(env));

    voters.contains_key(voter)
}

/// Adds a voter to the approved voter set.
///
/// Errors:
/// - `AlreadyInVotersList` if voter already exists
///
/// Security note:
/// - caller should enforce authorization before calling this helper.
pub fn write_add_voter(env: &Env, voter: &Address) -> Result<(), ContractError> {
    if read_is_voter(env, voter.clone()) {
        return Err(ContractError::AlreadyInVotersList);
    }

    let key = DataKey::VotersList;
    let mut voters: Map<Address, ()> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(Map::new(env));

    voters.set(voter.clone(), ());
    env.storage().persistent().set(&key, &voters);

    Ok(())
}

/// Removes a voter from the approved voter set.
///
/// Errors:
/// - `NotInVotersList` if voter does not exist
///
/// Security note:
/// - caller should enforce authorization before calling this helper.
pub fn write_remove_voter(env: &Env, voter: &Address) -> Result<(), ContractError> {
    if !read_is_voter(env, voter.clone()) {
        return Err(ContractError::NotInVotersList);
    }

    let key = DataKey::VotersList;
    let mut voters: Map<Address, ()> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(Map::new(env));

    voters.remove(voter.clone());
    env.storage().persistent().set(&key, &voters);

    Ok(())
}

/// Returns the full voter list.
pub fn read_voters_list(env: &Env) -> Result<Vec<Address>, ContractError> {
    let key = DataKey::VotersList;

    let voters: Map<Address, ()> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or_else(|| Map::new(env));

    Ok(voters.keys())
}

/// Returns the total number of registered voters.
fn read_voter_count(env: &Env) -> Result<u32, ContractError> {
    let voters = read_voters_list(env)?;
    Ok(voters.len())
}

/// Returns `(total_voters, passing_threshold)`.
///
/// Threshold is calculated as a rounded-up percentage using `VOTING_THRESHOLD`.
///
/// Example with 75:
/// - 1 voter  -> 1 required
/// - 2 voters -> 2 required
/// - 3 voters -> 3 required
/// - 4 voters -> 3 required
///
/// Errors:
/// - `NoVoters` if voter set is empty
/// - `Overflow` on arithmetic overflow
pub fn get_voter_info(env: &Env) -> Result<(u32, u32), ContractError> {
    let total = read_voter_count(env)?;

    if total == 0 {
        return Err(ContractError::NoVoters);
    }

    let threshold = total
        .checked_mul(VOTING_THRESHOLD)
        .and_then(|v| v.checked_add(99))
        .ok_or(ContractError::Overflow)?
        / 100;

    Ok((total, threshold))
}

/// Returns `(vote_count, has_passed)` for the current active proposal.
///
/// Errors:
/// - `NoPendingUpgradeAction` if there is no active proposal
///
/// Design note:
/// - because this module supports only one active proposal at a time,
///   `VotedList` is global to the active proposal.
pub fn read_has_upgrade_passed(e: &Env) -> Result<(u32, bool), ContractError> {
    let deadline = get_upgrade_voting_deadline(e);
    if deadline == 0 {
        return Err(ContractError::NoPendingUpgradeAction);
    }

    let (_, threshold) = get_voter_info(e)?;
    let key = DataKey::VotedList;

    let voted: Map<Address, ()> = e.storage().persistent().get(&key).unwrap_or(Map::new(e));
    let vote_count = voted.len();

    Ok((vote_count, vote_count >= threshold))
}
