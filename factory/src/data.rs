use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Managers,
    RegistryContract,
    WalletVersions,
    WalletVersion,
    PreviousVersion,
}
