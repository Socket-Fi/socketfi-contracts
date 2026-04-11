#![allow(unused)]
use crate::data::DataKey;
use socketfi_shared::utils::string_to_bytes;
use socketfi_shared::ContractError;
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env, Map, String, Vec};

pub fn write_latest_version(e: &Env, wasm: BytesN<32>) {
    let key = DataKey::WalletVersion;
    if let Some(pre) = read_latest_version(e) {
        write_previous_version(e, pre)
    }
    e.storage().instance().set(&key, &wasm);
}
pub fn write_previous_version(e: &Env, wasm: BytesN<32>) {
    let key = DataKey::PreviousVersion;

    e.storage().instance().set(&key, &wasm);
}

pub fn read_latest_version(e: &Env) -> Option<BytesN<32>> {
    let key = DataKey::WalletVersion;
    e.storage().instance().get(&key)
}
pub fn read_previous_version(e: &Env) -> Option<BytesN<32>> {
    let key = DataKey::PreviousVersion;
    e.storage().instance().get(&key)
}

pub fn write_create_wallet(
    e: &Env,
    passkey: &BytesN<77>,
    bls_keys: Vec<BytesN<96>>,
) -> Result<Address, ContractError> {
    let wasm = read_latest_version(&e).ok_or(ContractError::VersionNotFound)?;
    let mut salt = Bytes::new(e);
    salt.append(&passkey.to_xdr(e));
    let salt = e.crypto().sha256(&salt);
    let wallet_address = e
        .deployer()
        .with_current_contract(salt)
        .deploy_v2(wasm, (passkey, bls_keys, e.current_contract_address()));
    Ok(wallet_address)
}
