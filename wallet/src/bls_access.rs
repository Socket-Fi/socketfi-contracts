use soroban_sdk::{crypto::bls12_381::G1Affine, Address, BytesN, Env, Vec};

use crate::{data::DataKey, errors::WalletError};
use socketfi_shared::constants::MAX_BLS_KEYS;

/// Check whether the wallet has already been initialized.
///
/// Notes:
/// - Initialization is inferred from whether the aggregated BLS key
///   has been stored in persistent storage.
/// - Returns `true` once `DataKey::AggregatedBlsKey` exists.
pub fn is_initialized(e: &Env) -> bool {
    let key = DataKey::AggregatedBlsKey;
    e.storage().persistent().has(&key)
}

/// Read the external owner address from instance storage.
///
/// Notes:
/// - Returns `Some(Address)` if an owner has been set.
/// - Returns `None` if no owner is currently stored.
/// - Uses instance storage because owner data is contract instance state.
pub fn read_owner(e: &Env) -> Option<Address> {
    let key = DataKey::Owner;
    e.storage().instance().get(&key)
}

/// Write or replace the external owner address in instance storage.
///
/// Notes:
/// - Stores the provided owner address under `DataKey::Owner`.
/// - Overwrites any previously stored owner value.
pub fn write_owner(e: &Env, owner: &Address) {
    let key = DataKey::Owner;
    e.storage().instance().set(&key, owner);
}

/// Aggregate a list of BLS public keys into one aggregated key and store it.
///
/// Notes:
/// - Starts with the first key in the provided vector.
/// - Iteratively adds the remaining keys using BLS G1 point addition.
/// - Rejects input only when the number of keys exceeds `MAX_BLS_KEYS`.
/// - Stores the final aggregated public key in persistent storage.

pub fn write_agg_bls_key(env: &Env, bls_keys: Vec<BytesN<96>>) -> Result<(), WalletError> {
    let bls = env.crypto().bls12_381();

    let mut keypair_1_array = [0u8; 96];
    bls_keys
        .get_unchecked(0)
        .copy_into_slice(&mut keypair_1_array);

    let mut agg_pk = G1Affine::from_bytes(BytesN::from_array(env, &keypair_1_array));

    let n = bls_keys.len();

    if n > MAX_BLS_KEYS {
        return Err(WalletError::TooManyKeys);
    }

    for i in 1..n {
        let mut keypair_i_array = [0u8; 96];
        bls_keys
            .get_unchecked(i)
            .copy_into_slice(&mut keypair_i_array);

        let pk = G1Affine::from_bytes(BytesN::from_array(env, &keypair_i_array));
        agg_pk = bls.g1_add(&agg_pk, &pk);
    }

    env.storage()
        .persistent()
        .set(&DataKey::AggregatedBlsKey, &agg_pk.to_bytes());

    Ok(())
}

/// Read the aggregated BLS public key from persistent storage.
///
/// Notes:
/// - Returns `Some(BytesN<96>)` if an aggregated key has been stored.
/// - Returns `None` if the wallet has not yet stored an aggregated key.
pub fn read_aggregated_bls_key(e: &Env) -> Option<BytesN<96>> {
    let key = DataKey::AggregatedBlsKey;
    e.storage().persistent().get(&key)
}

/// Store the passkey payload in persistent storage.
///
/// Notes:
/// - Writes the provided passkey bytes under `DataKey::Passkey`.
/// - Overwrites any previously stored passkey value.
pub fn write_passkey(env: &Env, passkey: BytesN<77>) {
    env.storage().persistent().set(&DataKey::Passkey, &passkey);
}

/// Read the stored passkey payload from persistent storage.
///
/// Notes:
/// - Returns `Some(BytesN<77>)` if a passkey has been stored.
/// - Returns `None` if no passkey is currently set.
pub fn read_passkey(e: &Env) -> Option<BytesN<77>> {
    let key = DataKey::Passkey;
    e.storage().persistent().get(&key)
}
