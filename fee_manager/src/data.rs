use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    BaseFee,
    MaxPendingFee,
    FeeAssetRate(Address),
    DeferredFee(Address),
    PendingFee(Address),
}
