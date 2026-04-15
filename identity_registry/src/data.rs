use soroban_sdk::contracttype;

/// Storage keys for contract instance and persistent state.
///
/// Design notes:
/// - Each variant represents a distinct storage slot
/// - Used across instance and persistent storage
///
/// Audit notes:
/// - Keys must remain stable across upgrades
/// - Renaming or removing keys breaks backward compatibility
/// - Unused keys should be removed to avoid confusion
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// Validator set map: Map<BytesN<32>, ()> (persistent storage)
    Validators,
}
