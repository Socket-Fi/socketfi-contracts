#![allow(unused)]
use crate::data::DataKey;
use socketfi_shared::utils::string_to_bytes;
use socketfi_shared::ContractError;
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env, Map, String, Vec};
use upgrade::get_wallet_version;

pub fn write_create_wallet(
    e: &Env,
    passkey: &BytesN<77>,
    bls_keys: Vec<BytesN<96>>,
) -> Result<Address, ContractError> {
    let wasm = get_wallet_version(&e)?;
    let mut salt = Bytes::new(e);
    salt.append(&passkey.to_xdr(e));
    let salt = e.crypto().sha256(&salt);
    let wallet_address = e
        .deployer()
        .with_current_contract(salt)
        .deploy_v2(wasm, (passkey, bls_keys, e.current_contract_address()));
    Ok(wallet_address)
}
