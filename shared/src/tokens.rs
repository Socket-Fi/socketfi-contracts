use soroban_sdk::{contracttype, token, Address, Env, Map, Vec};

use crate::ContractError;
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    AllowanceExpiration,
    DefaultSpendLimit,
    SpendLimit(Address),
SupportedAssets

}

pub fn take_asset(env: &Env, from: &Address, asset: &Address, amount: i128) {
    let client = token::Client::new(env, asset);
    let to = env.current_contract_address();
    client.transfer(from, &to, &amount);
}

pub fn send_asset(env: &Env, to: &Address, asset: &Address, amount: i128) {
    let client = token::Client::new(env, asset);
    let from = env.current_contract_address();
    client.transfer(&from, to, &amount);
}

pub fn spend_asset(env: &Env, spender: &Address, asset: &Address, amount: i128, to: &Address) {
    let client = token::Client::new(env, asset);
    let from = env.current_contract_address();
    client.transfer_from(&spender, &from, to, &amount);
}

pub fn read_balance(env: &Env, asset: &Address) -> i128 {
    let client = token::Client::new(env, asset);
    let of = env.current_contract_address();
    client.balance(&of)
}

pub fn read_allowance(env: &Env, asset:  &Address, spender: &Address) -> i128 {
    let client = token::Client::new(env, asset);
    let from = env.current_contract_address();
    client.allowance(&from, spender)
}

pub fn write_approve(
    env: &Env,
    asset: &Address,
    spender: &Address,
    amount: &i128,
) -> Result<(), ContractError> {
    let client = token::Client::new(env, asset);
    let from = env.current_contract_address();

    let expiration = read_allowance_expiration(env)
        .checked_add(env.ledger().sequence())
        .ok_or(ContractError::InvalidExpiration)?;

    client.approve(&from, spender, amount, &expiration);
    Ok(())
}

pub fn write_allowance_expiration(env: &Env, ledger_offset: u32) {
    env.storage()
        .persistent()
        .set(&DataKey::AllowanceExpiration, &ledger_offset);
}

pub fn read_allowance_expiration(env: &Env) -> u32 {
    env.storage()
        .persistent()
        .get(&DataKey::AllowanceExpiration)
        .unwrap_or(17_000)
}

pub fn write_default_spend_limit(env: &Env, limit: i128) {
    env.storage()
        .instance()
        .set(&DataKey::DefaultSpendLimit, &limit);
}

pub fn read_default_spend_limit(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::DefaultSpendLimit)
        .unwrap_or(0)
}

pub fn read_limit(env: &Env, asset: Address) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::SpendLimit(asset))
        .unwrap_or(read_default_spend_limit(&env))
}

pub fn write_limit(env: &Env, asset: Address, limit: i128) {
    env.storage()
        .instance()
        .set(&DataKey::SpendLimit(asset), &limit);
}




/// Returns true if asset is supported for fee payment.
///
/// Audit notes:
/// - Defaults to false if storage key does not exist.
/// - Uses Map<Address, ()> as a set representation.
pub fn read_is_supported_asset(e: &Env, asset: Address) -> bool {
    e.storage()
        .persistent()
        .get::<_, Map<Address, ()>>(&DataKey::SupportedAssets)
        .map(|m| m.contains_key(asset))
        .unwrap_or(false)
}


pub fn read_supported_assets(e: &Env) -> Vec<Address> {
    let m = e
        .storage()
        .persistent()
        .get::<_, Map<Address, ()>>(&DataKey::SupportedAssets)
        .unwrap_or_else(|| Map::new(e));

    m.keys()
}

/// Adds a asset to the supported fee assets set.
///
/// Returns:
/// - Ok(()) if successfully added
/// - Err(ContractError::assetAlreadySupported) if already exists
///
/// Audit notes:
/// - Uses Map<Address, ()> as a set
/// - Must be protected by admin auth at higher level
pub fn write_is_supported_asset(
    e: &Env,
    asset: Address,
) -> Result<(), ContractError> {
    let mut m = e
        .storage()
        .persistent()
        .get::<_, Map<Address, ()>>(&DataKey::SupportedAssets)
        .unwrap_or_else(|| Map::new(e));

    if m.contains_key(asset.clone()) {
        return Err(ContractError::AssetAlreadySupported);
    }

    m.set(asset, ());
    e.storage().persistent().set(&DataKey::SupportedAssets, &m);

    Ok(())
}


pub fn write_not_supported_asset(
    e: &Env,
    asset: Address,
) -> Result<(), ContractError> {
    let mut m = e
        .storage()
        .persistent()
        .get::<_, Map<Address, ()>>(&DataKey::SupportedAssets)
        .unwrap_or_else(|| Map::new(e));

    if m.remove(asset).is_none() {
        return Err(ContractError::AssetNotSupported);
    }

    e.storage().persistent().set(&DataKey::SupportedAssets, &m);

    Ok(())
}