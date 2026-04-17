#![allow(unused)]
use socketfi_access::access::{read_fee_manager, read_registry, read_social_router};
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env, Map, String, Vec};
use upgrade::{errors::UpgradeError, get_wallet_version};

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

    // Build deterministic deployment salt from passkey.
    // Ensures stable address derivation for identical inputs.
    let mut salt = Bytes::new(e);
    salt.append(&passkey.to_xdr(e));
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