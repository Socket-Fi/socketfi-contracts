use crate::ContractError;
use soroban_sdk::{contracttype, Address, BytesN, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UpgradeType {
    Upgrade,
    WalletVersion,
}

impl UpgradeType {
    pub fn upgrade_type(s: String) -> Result<Self, ContractError> {
        let e = s.env();

        if s == String::from_str(&e, "upgrade") {
            return Ok(Self::Upgrade);
        }
        if s == String::from_str(&e, "wallet") {
            return Ok(Self::WalletVersion);
        }

        Err(ContractError::NotSupportedUpgradeType)
    }
}

#[derive(Clone)]
#[contracttype]
pub struct WebKeyDetails {
    pub passkey: BytesN<77>,
    pub username: String,
}

#[derive(Clone)]
#[contracttype]
pub struct ValidatorSignature {
    pub validator: BytesN<32>,
    pub signature: BytesN<64>,
}

#[derive(Clone)]
#[contracttype]
pub struct AccessSettings {
    pub default_allowance: i128,
    pub g_account: Option<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SocialPlatform {
    X,
    Discord,
    Telegram,
    Email,
    Tiktok,
    Sms,
}

impl SocialPlatform {
    pub fn is_platform_supported(s: String) -> Result<Self, ContractError> {
        let e = s.env();

        if s == String::from_str(&e, "x") || s == String::from_str(&e, "twitter") {
            return Ok(Self::X);
        }
        if s == String::from_str(&e, "discord") {
            return Ok(Self::Discord);
        }
        if s == String::from_str(&e, "telegram") {
            return Ok(Self::Telegram);
        }
        if s == String::from_str(&e, "email") {
            return Ok(Self::Email);
        }
        if s == String::from_str(&e, "tiktok") {
            return Ok(Self::Tiktok);
        }
        if s == String::from_str(&e, "sms") {
            return Ok(Self::Sms);
        }

        Err(ContractError::PlatformNotSupported)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::X => "x",
            Self::Discord => "discord",
            Self::Telegram => "telegram",
            Self::Email => "email",
            Self::Tiktok => "tiktok",
            Self::Sms => "sms",
        }
    }
}
