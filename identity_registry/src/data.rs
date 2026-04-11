use soroban_sdk::{contracttype, Address, Bytes, BytesN};


#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Managers,
    SoroswapContract,
    DappAdapterId,
    WalletVersions,
    WalletUsernameMap(Bytes),
    SupportedPlatform(Bytes),
    IsRegisteredUsername(Bytes, Bytes),
    UsernameSmartWalletMap(Bytes, Bytes),
    PasskeySmartWalletMap(BytesN<77>),
    IsSmartWallet(Address),
    WalletVersion,
    PreviousVersion,
}
