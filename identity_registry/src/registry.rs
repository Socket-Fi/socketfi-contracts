use soroban_sdk::{Address, BytesN, Env, String};

use socketfi_shared::utils::{passkey_wallet_key, userid_wallet_key, DataKey};
use socketfi_shared::ContractError;

/// Reads the wallet bound to a given `(platform, user_id)` identity.
///
/// Returns:
/// - `Ok(Some(Address))` if the identity has been registered
/// - `Ok(None)` if no mapping exists
///
/// Design notes:
/// - Key derivation is delegated to `socketfi_shared::utils::userid_wallet_key`,
///   ensuring a single canonical implementation across contracts.
///
/// Audit notes:
/// - Centralizing key derivation reduces risk of inconsistencies between contracts.
/// - Lookup correctness depends entirely on:
///   - consistent canonicalization (lowercase, no whitespace, etc.)
///   - identical validation rules being applied before both writes and reads
/// - Any divergence in validation or normalization between callers and the shared
///   key helper will result in silent lookup failure (`None`).
/// - Returning `Option<Address>` avoids forcing error-based control flow for existence checks.
pub fn read_userid_wallet_map(
    e: &Env,
    platform: String,
    user_id: String,
) -> Result<Option<Address>, ContractError> {
    let key = userid_wallet_key(e, platform, user_id)?;
    Ok(e.storage().persistent().get(&key))
}

/// Writes a new `(platform, user_id) -> wallet` mapping.
///
/// Write policy:
/// - first-write-only
/// - rebinding is explicitly rejected if the identity is already mapped
///
/// Returns:
/// - `Ok(())` on successful first-time registration
/// - `Err(ContractError::UseridAlreadyMapped)` if the identity already exists
///
/// Design notes:
/// - Uses shared key derivation logic to ensure consistency with read paths.
/// - Persistent storage is used because identity bindings are long-lived registry state.
///
/// Audit notes:
/// - Enforces immutability of identity binding at the storage layer.
/// - Prevents silent overwrites, which could otherwise lead to identity hijacking.
/// - If rebinding/recovery is required, it must be implemented via a separate,
///   explicitly authorized flow (e.g., validator-approved migration).
/// - Critical that `userid_wallet_key` internally enforces validation to avoid
///   bypass scenarios (e.g., invalid casing or hidden characters).
pub fn write_userid_wallet_map(
    e: &Env,
    platform: String,
    userid: String,
    wallet: Address,
) -> Result<(), ContractError> {
    let key = userid_wallet_key(e, platform, userid)?;

    // Prevent silent overwrite of an existing identity binding.
    if e.storage().persistent().has(&key) {
        return Err(ContractError::UseridAlreadyMapped);
    }

    e.storage().persistent().set(&key, &wallet);
    Ok(())
}

/// Writes a new `passkey -> wallet` mapping.
///
/// Write policy:
/// - first-write-only
/// - rebinding is explicitly rejected if the passkey is already mapped
///
/// Returns:
/// - `Ok(())` on successful first-time registration
/// - `Err(ContractError::PasskeyAlreadyMapped)` if the passkey already exists
///
/// Design notes:
/// - Uses shared key derivation (`passkey_wallet_key`) for consistency across contracts.
/// - Persistent storage is used to ensure passkey bindings remain durable.
///
/// Audit notes:
/// - Prevents overwriting existing passkey bindings, avoiding unauthorized reassignment.
/// - Assumes upstream authorization checks are enforced (e.g., factory or wallet auth).
/// - Correctness depends on strict passkey byte equality (no normalization layer).
pub fn write_passkey_wallet_map(
    e: &Env,
    passkey: BytesN<77>,
    wallet: Address,
) -> Result<(), ContractError> {
    let key = passkey_wallet_key(e, passkey)?;

    // Prevent silent overwrite of an existing passkey binding.
    if e.storage().persistent().has(&key) {
        return Err(ContractError::PasskeyAlreadyMapped);
    }

    e.storage().persistent().set(&key, &wallet);
    Ok(())
}

/// Reads the wallet bound to a given passkey.
///
/// Returns:
/// - `Ok(Some(Address))` if the passkey has been registered
/// - `Ok(None)` if no mapping exists
///
/// Design notes:
/// - Key derivation is delegated to shared utilities for consistency.
///
/// Audit notes:
/// - Lookup correctness depends on exact passkey byte match.
/// - No normalization is applied; any difference in passkey bytes results in a miss.
/// - Returning `Option<Address>` avoids forcing error handling for simple existence checks.
pub fn read_passkey_wallet_map(
    e: &Env,
    passkey: BytesN<77>,
) -> Result<Option<Address>, ContractError> {
    let key = passkey_wallet_key(e, passkey)?;

    Ok(e.storage().persistent().get::<DataKey, Address>(&key))
}
