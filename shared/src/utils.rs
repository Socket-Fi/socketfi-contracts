use soroban_sdk::{contracttype, Bytes, BytesN, Env, IntoVal, String, Val};

use crate::{
    constants::{DAY_IN_LEDGERS, MAX_LEN},
    types::SocialPlatform,
    ContractError,
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    UseridWalletMap(Bytes),
    PasskeyWalletMap(Bytes),
}

/// Validates a user identity string (`user_id`) used for platform bindings.
///
/// Validation rules:
/// - Must not be empty
/// - Must not exceed `MAX_LEN`
/// - Must not contain uppercase ASCII letters (A–Z)
/// - Must not contain any ASCII whitespace:
///   - space (32)
///   - tab (9)
///   - newline (10)
///   - vertical tab (11)
///   - form feed (12)
///   - carriage return (13)
///
/// Design notes:
/// - Uses raw UTF-8 bytes (`String -> Bytes`) instead of XDR to ensure validation
///   applies strictly to user input, not serialized metadata.
/// - Enforces canonical identity representation:
///   - lowercase-only (ASCII)
///   - no whitespace
///   - bounded length
///
/// Audit notes:
/// - This function is a **critical canonicalization layer** and MUST be used:
///   - before key derivation
///   - before signature message construction
///   - before identity comparison
///
/// - Failure to apply this consistently can lead to:
///   - mismatched storage keys
///   - signature verification failures
///   - duplicate logical identities (e.g. "Alice" vs "alice")
///
/// - This validator operates on raw UTF-8 bytes:
///   - Non-ASCII characters are currently allowed unless explicitly restricted
///   - If strict ASCII-only identities are required, enforce `v <= 127`
///
/// - Allowing Unicode introduces potential homoglyph risks:
///   e.g. visually similar characters representing different byte values
///
/// - `MAX_LEN` should be bounded (e.g. <= 128) to avoid gas abuse
///
/// - Uses `get_unchecked` for performance:
///   safe because iteration is strictly bounded by `len`
pub fn validate_userid(id: String) -> Result<(), ContractError> {
    let id_bytes: Bytes = id.into();
    let len = id_bytes.len();

    if len == 0 {
        return Err(ContractError::InvalidUserId);
    }

    if len > MAX_LEN {
        return Err(ContractError::MaxLengthExceeded);
    }

    for i in 0..len {
        let v = id_bytes.get_unchecked(i);

        if v >= 65 && v <= 90 {
            return Err(ContractError::UpperNotAllowed);
        }

        if matches!(v, 9 | 10 | 11 | 12 | 13 | 32) {
            return Err(ContractError::SpacesNotAllowed);
        }
    }

    Ok(())
}

/// Derives the storage key for a `(platform, user_id) -> wallet` mapping.
///
/// Design notes:
/// - Platform is normalized via `SocialPlatform::is_platform_supported`
///   ensuring only supported, canonical platform identifiers are used.
/// - `user_id` is validated before hashing to enforce canonical encoding.
/// - Raw bytes are used instead of XDR to avoid serialization artifacts.
/// - `0x00` separators are used between fields to prevent ambiguity.
/// - A domain separator (`"userid_wallet"`) isolates this key space.
///
/// Structure:
///   hash("userid_wallet" || 0x00 || platform || 0x00 || user_id)
///
/// Audit notes:
/// - Deterministic key derivation is guaranteed by:
///   - strict validation
///   - canonical platform normalization
///   - fixed ordering and separators
///
/// - `0x00` prevents concatenation ambiguity:
///   ("ab", "c") ≠ ("a", "bc")
///
/// - Hashing ensures:
///   - fixed-size storage keys
///   - no direct exposure of raw identities in keys
///
/// - Any modification to:
///   - validation rules
///   - platform normalization
///   - field order
///   will break compatibility with existing stored mappings.
///
/// - This function does NOT enforce rebinding policy.
///   That is handled at the storage write layer.
///
/// - Replay protection is NOT handled here.
///   Must be enforced at the signature/message layer.
pub fn userid_wallet_key(
    e: &Env,
    platform_str: String,
    user_id: String,
) -> Result<DataKey, ContractError> {
    let platform = SocialPlatform::is_platform_supported(platform_str)?;

    validate_userid(user_id.clone())?;

    let mut salt = Bytes::new(e);

    salt.append(&String::from_str(e, "userid_wallet").into());
    salt.push_back(0);

    salt.append(&String::from_str(e, platform.as_str()).into());
    salt.push_back(0);

    salt.append(&user_id.into());

    Ok(DataKey::UseridWalletMap(e.crypto().sha256(&salt).into()))
}

/// Derives the storage key for a `passkey -> wallet` mapping.
///
/// Design notes:
/// - Uses a distinct domain separator (`"passkey_wallet"`)
/// - Uses raw passkey bytes (`BytesN<77>`)
/// - Uses `0x00` separator for structured encoding
///
/// Audit notes:
/// - Ensures no overlap with user_id mapping namespace
/// - Passkey equality is strict byte equality (no normalization)
/// - Hashing ensures fixed-size key and avoids raw exposure
///
/// - Any change to encoding logic will break compatibility
pub fn passkey_wallet_key(e: &Env, passkey: BytesN<77>) -> Result<DataKey, ContractError> {
    let mut salt = Bytes::new(e);

    salt.append(&String::from_str(e, "passkey_wallet").into());
    salt.push_back(0);

    salt.append(&passkey.into_bytes());

    Ok(DataKey::PasskeyWalletMap(e.crypto().sha256(&salt).into()))
}

pub fn userid_payment_key(
    e: &Env,
    platform_str: String,
    user_id: String,
) -> Result<BytesN<32>, ContractError> {
    let platform = SocialPlatform::is_platform_supported(platform_str)?;

    validate_userid(user_id.clone())?;

    let mut salt = Bytes::new(e);

    salt.append(&String::from_str(e, "userid_wallet").into());
    salt.push_back(0);

    salt.append(&String::from_str(e, platform.as_str()).into());
    salt.push_back(0);

    salt.append(&user_id.into());

    Ok(e.crypto().sha256(&salt).into())
}

/// Extends TTL for contract instance storage.
///
/// Design notes:
/// - Keeps contract alive by refreshing TTL close to maximum
///
/// Audit notes:
/// - Must be called in functions relying on long-lived instance state
/// - Prevents unexpected contract eviction
/// - Uses buffer (`DAY_IN_LEDGERS`) to avoid edge expiry
///
/// - TTL strategy assumes periodic interaction with the contract
/// - Without interaction, state may still expire
pub fn bump_instance(e: &Env) {
    let max_ttl = e.storage().max_ttl();

    e.storage()
        .instance()
        .extend_ttl(max_ttl - DAY_IN_LEDGERS, max_ttl);
}

/// Extends TTL for a persistent storage entry.
///
/// Design notes:
/// - Used for long-lived mappings (e.g. identity registry)
///
/// Audit notes:
/// - Must be called on critical read/write paths
/// - Prevents silent expiration of mappings
///
/// - Failure to bump TTL may cause:
///   - loss of mappings
///   - inconsistent system behavior
///
/// - Overuse increases cost; apply selectively
pub fn bump_persistent<K>(e: &Env, key: &K)
where
    K: IntoVal<Env, Val>,
{
    let max_ttl = e.storage().max_ttl();

    e.storage()
        .persistent()
        .extend_ttl(key, max_ttl - DAY_IN_LEDGERS, max_ttl);
}
