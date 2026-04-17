use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub struct AccessSettings {
    pub default_allowance: i128,
    pub g_account: Option<Address>,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    FactoryContract,
    Owner,
    AggregatedBlsKey,
    Passkey,
    Nonce,
}
