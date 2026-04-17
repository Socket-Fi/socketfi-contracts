use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub struct CollectNowData {
    pub fee_asset: Address,
    pub total_fee_in_asset: i128,
    pub total_in_base: i128,
    pub total_tx_amount: i128,
}
#[derive(Clone)]
#[contracttype]
pub struct DeferData {
    pub updated_deferred_fee: i128,
    pub total_tx_amount: i128,
}

#[derive(Clone)]
#[contracttype]
pub enum FeeDecision {
    CollectNow(CollectNowData),
    Defer(DeferData),
}
