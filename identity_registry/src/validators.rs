use soroban_sdk::{BytesN, Env, Vec};

use crate::data::DataKey;
use socketfi_shared::ContractError;

/// Adds a validator public key to the validator set.
///
/// Write policy:
/// - first-write-only for a given validator key
///
/// Returns:
/// - `Ok(())` if validator was added
/// - `Err(ContractError::ValidatorAlreadyExists)` if already present
///
/// Design notes:
/// - Validators are stored as keys in a `Map<BytesN<32>, ()>`.
/// - Unit `()` is used because only membership matters; no per-validator metadata is stored.
///
/// Audit notes:
/// - Full validator map is loaded, modified, and written back on each update.
///   This is acceptable for small validator sets but cost grows with set size.
/// - Persistent storage TTL must be maintained elsewhere for long-lived validator state.
/// - No maximum validator count is enforced here.
pub fn write_add_validator(e: &Env, v: BytesN<32>) -> Result<(), ContractError> {
    let mut m = e
        .storage()
        .persistent()
        .get::<_, soroban_sdk::Map<BytesN<32>, ()>>(&DataKey::Validators)
        .unwrap_or_else(|| soroban_sdk::Map::new(e));

    if m.contains_key(v.clone()) {
        return Err(ContractError::ValidatorAlreadyExists);
    }

    m.set(v, ());
    e.storage().persistent().set(&DataKey::Validators, &m);
    Ok(())
}

/// Removes a validator public key from the validator set.
///
/// Returns:
/// - `Ok(())` if validator existed and was removed
/// - `Err(ContractError::ValidatorNotFound)` if not present
///
/// Audit notes:
/// - Because signature requirement is currently derived from validator count,
///   removing a validator immediately changes the effective signature policy.
/// - Consider whether removal of the final validator should be allowed,
///   since an empty validator set disables identity binding.
pub fn write_remove_validator(e: &Env, v: BytesN<32>) -> Result<(), ContractError> {
    let mut m = e
        .storage()
        .persistent()
        .get::<_, soroban_sdk::Map<BytesN<32>, ()>>(&DataKey::Validators)
        .unwrap_or_else(|| soroban_sdk::Map::new(e));

    if m.remove(v).is_none() {
        return Err(ContractError::ValidatorNotFound);
    }

    e.storage().persistent().set(&DataKey::Validators, &m);
    Ok(())
}

/// Returns whether a given public key is currently in the validator set.
///
/// Returns:
/// - `true` if validator exists
/// - `false` otherwise
///
/// Audit notes:
/// - Read path treats missing validator map as empty set.
pub fn read_is_validator(e: &Env, v: BytesN<32>) -> bool {
    e.storage()
        .persistent()
        .get::<_, soroban_sdk::Map<BytesN<32>, ()>>(&DataKey::Validators)
        .map(|m| m.contains_key(v))
        .unwrap_or(false)
}

/// Returns the list of validator public keys.
///
/// Returns:
/// - `Ok(Vec<BytesN<32>>)` if validator map exists
/// - `Err(ContractError::ValidatorNotFound)` if validator map does not exist
///
/// Audit notes:
/// - Current behavior treats “no validator map stored” as an error rather than an empty set.
/// - Depending on API preference, returning an empty vector may be more ergonomic for callers.
pub fn read_validators(e: &Env) -> Result<Vec<BytesN<32>>, ContractError> {
    let m = e
        .storage()
        .persistent()
        .get::<_, soroban_sdk::Map<BytesN<32>, ()>>(&DataKey::Validators)
        .ok_or(ContractError::ValidatorNotFound)?;
    Ok(m.keys())
}

/// Returns the current required signature count.
///
/// Current behavior:
/// - equal to the number of validators in the set
/// - returns `0` if validator map does not exist
///
/// Audit notes:
/// - This is not a configurable threshold in the usual multisig sense.
///   It currently implements an “all validators must sign” policy.
/// - If true threshold semantics are desired (e.g. 2-of-3),
///   threshold should be stored separately from validator count.
/// - Renaming this function may improve clarity for reviewers and integrators.
pub fn read_threshold(e: &Env) -> u32 {
    e.storage()
        .persistent()
        .get::<_, soroban_sdk::Map<BytesN<32>, ()>>(&DataKey::Validators)
        .map(|m| m.len())
        .unwrap_or(0)
}
