#![allow(unused)]
use socketfi_access::access::{read_fee_manager, read_registry, read_social_router};
use soroban_sdk::{
    crypto::bls12_381::G1Affine, xdr::ToXdr, Address, Bytes, BytesN, Env, Map, String, Vec,
};
use upgrade::{errors::UpgradeError, get_wallet_version};

// Aggregates all BLS public keys into a single canonical aggregate key.
// BLS aggregation is order-independent, meaning the resulting aggregate
// public key remains identical regardless of the ordering of `bls_keys`.
//
// This aggregate key is used as part of wallet address derivation to ensure
// the deployed wallet address commits to the intended BLS signer set rather
// than the passkey alone.
fn write_agg_bls_key(env: &Env, bls_keys: Vec<BytesN<96>>) -> BytesN<96> {
    let bls = env.crypto().bls12_381();

    let mut first_array = [0u8; 96];
    bls_keys.get_unchecked(0).copy_into_slice(&mut first_array);

    let mut agg_pk = G1Affine::from_bytes(BytesN::from_array(env, &first_array));

    let n = bls_keys.len();

    for i in 1..n {
        let mut key_array = [0u8; 96];

        bls_keys.get_unchecked(i).copy_into_slice(&mut key_array);

        let pk = G1Affine::from_bytes(BytesN::from_array(env, &key_array));

        agg_pk = bls.g1_add(&agg_pk, &pk);
    }

    agg_pk.to_bytes()
}

/// Deploy a new wallet contract instance.
///
/// Notes:
/// - Uses the currently approved wallet wasm hash.
/// - Derives deployment salt from the provided passkey.
/// - Passes dependencies and auth data into wallet constructor.
/// - registry , fee_manager and social router are pre-configured (uses `unwrap()`).
pub fn write_create_wallet(
    e: &Env,
    passkey: &BytesN<77>,
    bls_keys: Vec<BytesN<96>>,
) -> Result<Address, UpgradeError> {
    // Load the approved wallet wasm version for deployment.
    let wasm = get_wallet_version(&e).unwrap();

    let agg = write_agg_bls_key(e, bls_keys.clone());

    // Build deterministic deployment salt from passkey and aggregate canonical bls key.
    // Ensures stable address derivation for identical inputs.
    let mut salt = Bytes::new(e);
    salt.append(&passkey.to_xdr(e));
    salt.append(&agg.to_xdr(e));
    let salt = e.crypto().sha256(&salt);

    // Deploy wallet contract with constructor arguments:
    // (passkey, bls_keys, registry, social_router, fee_manager, factory)
    let wallet_address = e.deployer().with_current_contract(salt).deploy_v2(
        wasm,
        (
            passkey,
            bls_keys,
            read_registry(e).unwrap(),
            read_social_router(e).unwrap(),
            read_fee_manager(e),
            e.current_contract_address(),
        ),
    );

    Ok(wallet_address)
}
