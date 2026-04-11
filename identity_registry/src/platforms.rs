use soroban_sdk::{Address, BytesN, Env, String};

use crate::{data::DataKey, error::ContractError, formatter::string_to_bytes};

pub fn read_is_smart_wallet(e: &Env, wallet_id: Address) -> bool {
    let key = DataKey::IsSmartWallet(wallet_id);
    e.storage().persistent().has(&key)
}

pub fn write_is_smart_wallet(e: &Env, wallet_id: Address) {
    let key = DataKey::IsSmartWallet(wallet_id.clone());
    e.storage().persistent().set(&key, &true);
}

pub fn is_registered_username(e: &Env, username: String) -> Result<bool, ContractError> {
    let key = DataKey::WalletUsernameMap(string_to_bytes(e, username)?);
    Ok(e.storage().persistent().has(&key))
}

pub fn write_username_wallet_map(
    e: &Env,
    username: String,
    wallet_address: Address,
) -> Result<(), ContractError> {
    if is_registered_username(e, username.clone())? {
        return Err(ContractError::UsernameAlreadyRegistered);
    }
    let key = DataKey::WalletUsernameMap(string_to_bytes(e, username)?);
    e.storage().persistent().set(&key, &wallet_address);
    Ok(())
}

pub fn read_username_wallet_map(e: &Env, username: String) -> Result<Address, ContractError> {
    let key = DataKey::WalletUsernameMap(string_to_bytes(e, username)?);

    e.storage()
        .persistent()
        .get::<DataKey, Address>(&key)
        .ok_or(ContractError::UsernameNotRegistered)
}

pub fn is_linked_passkey(e: &Env, passkey: BytesN<77>) -> bool {
    let key = DataKey::PasskeySmartWalletMap(passkey);
    e.storage().instance().has(&key)
}

pub fn write_passkey_wallet_map(
    e: &Env,
    passkey: BytesN<77>,
    wallet_address: Address,
) -> Result<(), ContractError> {
    if is_linked_passkey(e, passkey.clone()) {
        return Err(ContractError::PasskeyAlreadyLinked);
    }
    let key = DataKey::PasskeySmartWalletMap(passkey);
    e.storage().instance().set(&key, &wallet_address);
    Ok(())
}

pub fn read_passkey_wallet_map(e: &Env, passkey: BytesN<77>) -> Result<Address, ContractError> {
    if !is_linked_passkey(e, passkey.clone()) {
        return Err(ContractError::PasskeyNotLinked);
    }
    let key = DataKey::PasskeySmartWalletMap(passkey);

    e.storage()
        .instance()
        .get::<DataKey, Address>(&key)
        .ok_or(ContractError::WalletNotFound)
}
