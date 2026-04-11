use soroban_sdk::{Address, Env};

use crate::{data::DataKey, error::ContractError};

pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_admin(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::Admin;
    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::AdminNotFound)
}

pub fn write_admin(e: &Env, admin: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, admin);
}

pub fn authenticate_admin(e: &Env) -> Result<(), ContractError> {
    let admin = read_admin(e)?;
    admin.require_auth();

    Ok(())
}
