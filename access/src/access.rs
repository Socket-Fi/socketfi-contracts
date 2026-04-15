use socketfi_shared::ContractError;
use soroban_sdk::{contracttype, Address, Bytes, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Factory,
    Registry,
    FeeManager,
    SocialPayments,
    PaymentManager,
    UseridWalletMap(Bytes),
    PasskeyWalletMap(Bytes),
}

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

/// Reads the configured social_payments contract address.
///
/// Returns:
/// - `Ok(Address)` if social_payments is set
/// - `Err(ContractError::SocialPaymentsNotFound)` otherwise
///
/// Audit notes:
/// - Reads from instance storage; instance TTL must be maintained.
pub fn read_social_payments(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::SocialPayments;
    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::SocialPayNotFound)
}

/// Writes the configured social_payments contract address.
///
/// Design notes:
/// - Low-level storage helper only.
/// - Does not perform authorization checks.
///
/// Audit notes:
/// - Must only be called from trusted/admin-controlled flows.
/// - No business-rule validation is enforced here
///   (e.g. uniqueness vs admin/factory or other address constraints).
pub fn write_social_payments(e: &Env, social_payment: &Address) {
    let key = DataKey::SocialPayments;
    e.storage().instance().set(&key, social_payment);
}

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