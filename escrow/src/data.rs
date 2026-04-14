use soroban_sdk::contracttype;

/// Storage keys used by the factory contract.
///
/// Stored in instance storage because these values are:
/// - small
/// - global to the contract instance
/// - frequently read during normal execution
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// Factory administrator with authority over config and governance actions.
    Admin,

    /// Address of the registry / identity registry contract.
    Registry,

    /// Address of the fee manager contract.
    FeeManager,
}
