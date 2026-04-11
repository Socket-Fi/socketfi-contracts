#![warn(dead_code)]
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AdminNotFound = 91,
    AlreadyInitialized = 93,
    VersionNotFound = 117,
    UsernameAlreadyRegistered = 443,
    UsernameNotRegistered = 445,
    PasskeyAlreadyLinked = 447,
    PasskeyNotLinked = 449,
    UpperNotAlloyed = 743,
    MaxLengthExceeded = 745,
    WalletNotFound = 747,
}
