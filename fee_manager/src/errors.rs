use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    UnsupportedAsset = 417,
    MaxPendingFeeNotFound = 423,
    FeeRateNotSet = 425,
    InvalidAmount = 729,
}
