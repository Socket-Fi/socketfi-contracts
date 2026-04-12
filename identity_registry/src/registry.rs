use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env, String};

use crate::data::DataKey;
use socketfi_shared::ContractError;
use socketfi_shared::{
    types::SocialPlatform,
    utils::{check_lower, string_to_bytes},
};

pub fn read_is_smart_wallet(e: &Env, wallet_id: Address) -> bool {
    let key = DataKey::IsSmartWallet(wallet_id);
    e.storage().persistent().has(&key)
}

pub fn write_is_smart_wallet(e: &Env, wallet_id: Address) {
    let key = DataKey::IsSmartWallet(wallet_id.clone());
    e.storage().persistent().set(&key, &true);
}

fn username_wallet_key(
    e: &Env,
    platform_str: String,
    username: String,
) -> Result<DataKey, ContractError> {
    let platform = SocialPlatform::is_platform_supported(platform_str)?;
    check_lower(e, username.clone())?;

    let mut salt = Bytes::new(e);
    salt.append(&String::from_str(e, platform.as_str()).to_xdr(e));
    salt.append(&string_to_bytes(e, username)?);

    Ok(DataKey::UsernameWalletMap(e.crypto().sha256(&salt).into()))
}
fn passkey_wallet_key(e: &Env, passkey: BytesN<77>) -> Result<DataKey, ContractError> {
    let mut salt = Bytes::new(e);

    salt.append(&passkey.into_bytes());

    Ok(DataKey::UsernameWalletMap(e.crypto().sha256(&salt).into()))
}

pub fn read_username_wallet_map(
    e: &Env,
    platform: String,
    username: String,
) -> Result<Address, ContractError> {
    let key = username_wallet_key(e, platform, username)?;
    e.storage()
        .persistent()
        .get(&key)
        .ok_or(ContractError::UsernameNotRegistered)
}

pub fn read_is_mapped_username(
    e: &Env,
    platform: String,
    username: String,
) -> Result<bool, ContractError> {
    let key = username_wallet_key(e, platform, username)?;
    Ok(e.storage().persistent().has(&key))
}

pub fn write_username_wallet_map(
    e: &Env,
    platform: String,
    username: String,
    wallet: Address,
) -> Result<(), ContractError> {
    let key = username_wallet_key(e, platform, username)?;

    if e.storage().persistent().has(&key) {
        return Err(ContractError::UsernameAlreadyMapped);
    }

    e.storage().persistent().set(&key, &wallet);
    Ok(())
}

pub fn write_passkey_wallet_map(
    e: &Env,
    passkey: BytesN<77>,
    wallet: Address,
) -> Result<(), ContractError> {
    let key = passkey_wallet_key(e, passkey)?;
    if e.storage().persistent().has(&key) {
        return Err(ContractError::PasskeyAlreadyMapped);
    }
    e.storage().instance().set(&key, &wallet);
    Ok(())
}

pub fn read_passkey_wallet_map(e: &Env, passkey: BytesN<77>) -> Result<Address, ContractError> {
    let key = passkey_wallet_key(e, passkey)?;

    e.storage()
        .instance()
        .get::<DataKey, Address>(&key)
        .ok_or(ContractError::WalletNotFound)
}
