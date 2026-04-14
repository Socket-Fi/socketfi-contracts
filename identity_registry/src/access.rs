use soroban_sdk::{Address, Env};

use crate::data::DataKey;
use socketfi_shared::ContractError;

/// Returns `true` if the contract admin has already been initialized.
///
/// Design notes:
/// - `Admin` acts as the initialization marker for this contract.
/// - Used to prevent constructor re-entry / double initialization.
///
/// Audit notes:
/// - This assumes initialization is atomic:
///   if `Admin` exists, the contract is treated as initialized.
/// - If future initialization logic becomes multi-step, relying only on `Admin`
///   as the initialization marker may be insufficient.
pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

/// Reads the configured contract admin.
///
/// Returns:
/// - `Ok(Address)` if admin is set
/// - `Err(ContractError::AdminNotFound)` if not initialized
///
/// Audit notes:
/// - Reads from instance storage, so contract instance TTL must be maintained
///   to avoid accidental expiration of admin state.
pub fn read_admin(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::Admin;
    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::AdminNotFound)
}

/// Writes the contract admin to instance storage.
///
/// Design notes:
/// - Low-level storage helper only.
/// - Does not perform authorization checks.
///
/// Audit notes:
/// - Must only be called from trusted flows such as:
///   - constructor
///   - authenticated admin update paths
/// - Misuse of this helper in an unprotected path would compromise admin control.
pub fn write_admin(e: &Env, admin: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, admin);
}

/// Requires authorization from the currently configured admin.
///
/// Returns:
/// - `Ok(())` if the stored admin successfully authorizes
/// - `Err(ContractError::AdminNotFound)` if admin is not configured
///
/// Audit notes:
/// - Security depends on `read_admin` returning the correct stored admin.
/// - This function does not bump TTL; callers should ensure instance TTL is
///   maintained elsewhere for long-lived contract configuration.
pub fn authenticate_admin(e: &Env) -> Result<(), ContractError> {
    let admin = read_admin(e)?;
    admin.require_auth();

    Ok(())
}

/// Reads the configured factory contract address.
///
/// Returns:
/// - `Ok(Address)` if factory is set
/// - `Err(ContractError::FactoryNotFound)` otherwise
///
/// Audit notes:
/// - Reads from instance storage; instance TTL must be maintained.
pub fn read_factory(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::Factory;
    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::FactoryNotFound)
}

/// Writes the configured factory contract address.
///
/// Design notes:
/// - Low-level storage helper only.
/// - Does not perform authorization checks.
///
/// Audit notes:
/// - Must only be called from trusted/admin-controlled flows.
/// - No business-rule validation is enforced here
///   (e.g. whether factory equals another configured address).
pub fn write_factory(e: &Env, factory: &Address) {
    let key = DataKey::Factory;
    e.storage().instance().set(&key, factory);
}

/// Reads the configured escrow contract address.
///
/// Returns:
/// - `Ok(Address)` if escrow is set
/// - `Err(ContractError::EscrowNotFound)` otherwise
///
/// Audit notes:
/// - Reads from instance storage; instance TTL must be maintained.
pub fn read_escrow(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::Escrow;
    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::EscrowNotFound)
}

/// Writes the configured escrow contract address.
///
/// Design notes:
/// - Low-level storage helper only.
/// - Does not perform authorization checks.
///
/// Audit notes:
/// - Must only be called from trusted/admin-controlled flows.
/// - No business-rule validation is enforced here
///   (e.g. uniqueness vs admin/factory or other address constraints).
pub fn write_escrow(e: &Env, escrow: &Address) {
    let key = DataKey::Escrow;
    e.storage().instance().set(&key, escrow);
}
