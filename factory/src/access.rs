use soroban_sdk::{Address, Env};

use crate::data::DataKey;
use socketfi_shared::ContractError;

// ---------------------------------------------------------------------
// Existence checks
// ---------------------------------------------------------------------

/// Returns `true` if the factory admin has already been initialized.
pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

// ---------------------------------------------------------------------
// Admin
// ---------------------------------------------------------------------

/// Reads the current admin address from instance storage.
pub fn read_admin(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::Admin;

    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::AdminNotFound)
}

/// Writes the admin address to instance storage.
///
/// Audit note:
/// - caller must enforce authorization where required
pub fn write_admin(e: &Env, admin: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, admin);
}

// ---------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------

/// Reads the configured registry contract address.
pub fn read_registry(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::Registry;

    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::RegistryNotFound)
}

/// Writes the registry contract address to instance storage.
///
/// Audit note:
/// - caller must enforce authorization where required
pub fn write_registry(e: &Env, registry: &Address) {
    let key = DataKey::Registry;
    e.storage().instance().set(&key, registry);
}

// ---------------------------------------------------------------------
// Fee manager
// ---------------------------------------------------------------------

/// Reads the configured fee manager contract address.
pub fn read_fee_manager(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::FeeManager;

    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::FeeManagerNotFound)
}

/// Writes the fee manager contract address to instance storage.
///
/// Audit note:
/// - caller must enforce authorization where required
pub fn write_fee_manager(e: &Env, fee_manager: &Address) {
    let key = DataKey::FeeManager;
    e.storage().instance().set(&key, fee_manager);
}

// ---------------------------------------------------------------------
// Authorization
// ---------------------------------------------------------------------

/// Authenticates the currently configured admin.
pub fn authenticate_admin(e: &Env) -> Result<(), ContractError> {
    let admin = read_admin(e)?;
    admin.require_auth();
    Ok(())
}
