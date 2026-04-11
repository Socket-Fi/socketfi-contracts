use soroban_sdk::{contracttype, Address, BytesN, String};

#[derive(Clone)]
#[contracttype]
pub struct WebKeyDetails {
    pub passkey: BytesN<77>,
    pub username: String,
}

#[derive(Clone)]
#[contracttype]
pub struct AccessSettings {
    pub default_allowance: i128,
    pub g_account: Option<Address>,
}
