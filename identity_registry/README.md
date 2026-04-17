# Identity Registry Contract

A Soroban smart contract for managing secure mappings between social identities and wallet addresses.  
This contract serves as the canonical identity resolution layer within the SocketFi ecosystem.

---

## Overview

The Identity Registry enables:

- Mapping `(platform, user_id)` → `wallet address`
- Validator-backed identity verification
- Cross-contract wallet resolution
- Secure and deterministic identity binding

It is designed to be lightweight, composable, and safe for production use.

---

## Features

- Deterministic identity mapping
- Multi-validator signature verification
- Duplicate validator protection
- Cross-contract compatibility
- Efficient persistent storage
- Clean error handling

---

## Data Model

### Identity Key

```
(platform: String, user_id: String)
```

### Stored Value

```
Address (wallet)
```

---

## Core Functions

### Bind Identity

```rust
fn bind_identity(
    wallet: Address,
    user_id: String,
    platform: String,
    signatures: Vec<ValidatorSignature>,
) -> Result<(), ContractError>
```

- Requires wallet authorization
- Validates signatures against registered validators
- Enforces signature threshold
- Prevents duplicate validators
- Stores identity mapping

---

### Get Wallet by User ID

```rust
fn get_wallet_by_userid(
    platform: String,
    user_id: String
) -> Option<Address>
```

- Returns wallet address if mapping exists
- Used by external contracts for identity resolution

---

## Security Model

- **Threshold Enforcement**: Requires exact number of validator signatures
- **Validator Verification**: Only approved validators are accepted
- **Duplicate Protection**: Prevents repeated validator usage
- **Authorization Required**: Wallet must approve identity binding
- **Deterministic Messages**: Prevents replay inconsistencies

---

## Integration

### Used By

- Wallet Contract → resolve user identities
- Payment Contract → route payments via user_id
- Factory Contract → verify identity during wallet creation

---

### Example Cross-Contract Call

```rust
let wallet: Option<Address> = e.invoke_contract(
    &registry,
    &Symbol::new(e, "get_wallet_by_userid"),
    args,
)?;
```

---

## Error Handling

Typical errors include:

- `IncorrectNumberOfSignatures`
- `NotValidator`
- `DuplicateValidator`
- `IdentityAlreadyExists`
- `InvalidPlatform`

---

## Storage

- Uses Soroban persistent storage
- Keyed by deterministic identity keys
- Optimized for frequent reads

---

## Design Notes

- Stateless verification logic
- Deterministic XDR-based message construction
- Minimal storage overhead
- Designed for composability across contracts

---

## Audit Considerations

- Ensure validator threshold cannot be bypassed
- Verify message construction consistency
- Prevent replay attacks
- Enforce strict platform validation
- Prevent overwriting existing identities

---

## Future Improvements

- Identity updates / rebinding
- Expiring identity mappings
- Platform registry standardization
- Signature aggregation (e.g., BLS)

---

## License

MIT
