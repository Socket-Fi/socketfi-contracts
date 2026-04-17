use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum WalletError {
    InvalidSignature = 99,
    AlreadyInitialized = 411,
    ExceedMaxAllowance = 719,
    InvalidLimit = 723,
    InvalidAmount = 729,
    InvalidInvokeContract = 735,
    InvalidInvokeFunction = 737,
    TooManyKeys = 739,
}
