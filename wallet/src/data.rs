use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    FactoryContract,
    Owner,
    AggregatedBlsKey,
    WebKeys,
    AllowanceExpiration,
    DefaultSpendLimit,
    SpendLimit(Address),
    Nonce,
}
