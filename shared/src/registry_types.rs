use crate::registry_errors::RegistryError;
use soroban_sdk::{contracttype, BytesN, String};

// -----------------------------------------------------------------------------
// Validator Signature
// -----------------------------------------------------------------------------

/// Represents a validator's signature over a message.
///
/// FIELDS:
/// - `validator`: identifier of validator (typically public key or hash)
/// - `signature`: signature bytes (expected 64 bytes)
///
/// ASSUMPTIONS:
/// - Signature scheme is fixed (e.g. Ed25519, BLS, etc.)
/// - Validation logic is handled elsewhere (this is just a data container)
///
/// IMPORTANT:
/// - Lengths are fixed:
///     - validator: 32 bytes
///     - signature: 64 bytes
/// - No validation is performed here
#[derive(Clone)]
#[contracttype]
pub struct ValidatorSignature {
    pub validator: BytesN<32>,
    pub signature: BytesN<64>,
}

// -----------------------------------------------------------------------------
// Social Platform Enum
// -----------------------------------------------------------------------------

/// Supported social identity platforms.
///
/// DESIGN:
/// - Used for identity binding (wallet ↔ platform ↔ user_id)
/// - Stored and compared in normalized lowercase string form
///
/// IMPORTANT:
/// - Enum is persisted via contracttype → must remain backward compatible
/// - Do NOT reorder variants in future upgrades
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
    /// Parses a string into a supported `SocialPlatform`.
    ///
    /// INPUT:
    /// - Expected lowercase values:
    ///     "x", "discord", "telegram", "email", "tiktok", "sms"
    ///
    /// RETURNS:
    /// - Ok(SocialPlatform) → valid platform
    /// - Err(PlatformNotSupported) → invalid input
    ///
    /// IMPORTANT:
    /// - Matching is STRICT and case-sensitive.
    /// - "X", "Discord", etc. will FAIL.
    ///
    /// SECURITY:
    /// - Prevents unsupported platforms from entering system state.
    ///
    /// DESIGN ASSUMPTION:
    /// - Caller normalizes input before calling (e.g. frontend or API layer).
    pub fn is_platform_supported(s: String) -> Result<Self, RegistryError> {
        let e = s.env();

        if s == String::from_str(&e, "x") {
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

        Err(RegistryError::PlatformNotSupported)
    }

    /// Returns the canonical string representation of the platform.
    ///
    /// NOTE:
    /// - Always returns lowercase string
    /// - Matches exactly the expected input for `is_platform_supported`
    ///
    /// IMPORTANT:
    /// - This ensures consistency across:
    ///     - storage keys
    ///     - hashing / message signing
    ///     - off-chain integrations
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
