#![allow(unused)]
use crate::{data::BlsKeyWithProof, errors::FactoryError};
use socketfi_access::access::{read_fee_manager, read_registry, read_social_router};
use socketfi_shared::{bls::g1_group_gen_point, constants::DST};
use soroban_sdk::{
    crypto::bls12_381::{G1Affine, G2Affine},
    vec,
    xdr::ToXdr,
    Address, Bytes, BytesN, Env, Vec,
};
use upgrade::get_wallet_version;

pub fn extract_bls_keys(e: &Env, bls_keys_pop: Vec<BlsKeyWithProof>) -> Vec<BytesN<96>> {
    let mut bls_keys: Vec<BytesN<96>> = Vec::new(e);

    for bls_key_pop in bls_keys_pop.iter() {
        bls_keys.push_back(bls_key_pop.key.clone());
    }

    bls_keys
}

pub fn read_pop_salt(
    e: &Env,
    passkey: &BytesN<77>,
    bls_keys: Vec<BytesN<96>>,
) -> Result<BytesN<32>, FactoryError> {
    if bls_keys.is_empty() {
        return Err(FactoryError::MissingBlsKeys);
    }

    let agg = read_agg_bls_key(e, bls_keys);
    let mut salt = Bytes::new(e);
    salt.append(&passkey.to_xdr(e));
    salt.append(&agg.clone().to_xdr(e));
    let salt = e.crypto().sha256(&salt);
    Ok(BytesN::from(salt))
}

// Aggregates all BLS public keys into a single canonical aggregate key.
// BLS aggregation is order-independent, meaning the resulting aggregate
// public key remains identical regardless of the ordering of `bls_keys`.
//
// This aggregate key is used as part of wallet address derivation to ensure
// the deployed wallet address commits to the intended BLS signer set rather
// than the passkey alone.
fn read_agg_bls_key(env: &Env, bls_keys: Vec<BytesN<96>>) -> BytesN<96> {
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

// Verifies proof-of-possession for a single BLS public key.
// Each signer must sign the wallet deployment salt, which is derived from
// the passkey and aggregate BLS key. This proves each submitted public key
// is controlled by its corresponding signer before it is accepted into the
// wallet authentication set.
fn validate_a_bls_key_pop(
    e: &Env,
    salt: BytesN<32>,
    bls_key_pop: BlsKeyWithProof,
) -> Result<(), FactoryError> {
    // Access BLS12-381 operations from the Soroban crypto interface.
    let bls = e.crypto().bls12_381();

    let dst: Bytes = Bytes::from_slice(&e, DST.as_bytes());

    // Load the negative G1 generator used in the pairing equation.
    let neg_g1 = G1Affine::from_bytes(g1_group_gen_point(e));

    // Hash the payload into a point in G2 using the configured DST.
    let msg_g2 = bls.hash_to_g2(&salt.into(), &dst);

    // Prepare the two input vectors for pairing verification.
    let vp1 = vec![&e, G1Affine::from_bytes(bls_key_pop.key), neg_g1];
    let vp2 = vec![&e, msg_g2, G2Affine::from_bytes(bls_key_pop.sig)];

    // Signature is valid only if the pairing equation holds.
    if !bls.pairing_check(vp1, vp2) {
        return Err(FactoryError::InvalidPoPSignature);
    }
    Ok(())
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
    bls_keys_pop: Vec<BlsKeyWithProof>,
) -> Result<Address, FactoryError> {
    // Load the approved wallet wasm version for deployment.
    let wasm = get_wallet_version(&e).unwrap();

    let bls_keys = extract_bls_keys(e, bls_keys_pop.clone());

    let salt = read_pop_salt(e, passkey, bls_keys.clone())?;

    for bls_key_pop in bls_keys_pop.iter() {
        validate_a_bls_key_pop(e, salt.clone(), bls_key_pop)?;
    }

    // Build deterministic deployment salt from passkey and aggregate canonical bls key.
    // Ensures stable address derivation for identical inputs.

    // Deploy wallet contract with constructor arguments:
    // (passkey, bls_keys, registry, social_router, fee_manager, factory)
    let wallet_address = e.deployer().with_current_contract(salt).deploy_v2(
        wasm,
        (
            passkey,
            bls_keys.clone(),
            read_registry(e).unwrap(),
            read_social_router(e).unwrap(),
            read_fee_manager(e).unwrap(),
            e.current_contract_address(),
        ),
    );

    Ok(wallet_address)
}
