use soroban_sdk::{Address, Env};

use crate::data::DataKey;
use socketfi_shared::ContractError;
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

pub fn read_registry(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::RegistryContract;
    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::RegistryNotFound)
}

pub fn write_registry(e: &Env, registry: &Address) {
    let key = DataKey::RegistryContract;
    e.storage().instance().set(&key, registry);
}

pub fn authenticate_admin(e: &Env) -> Result<(), ContractError> {
    let admin = read_admin(e)?;
    admin.require_auth();

    Ok(())
}
