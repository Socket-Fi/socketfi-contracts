# Social Payment Router Contract

The **Social Payment Router** enables identity-based payments by routing transactions using social identifiers instead of wallet addresses.

It bridges user identity (e.g. X, email, Telegram) with on-chain wallets via the Identity Registry.

---

## Overview

The Social Payment Router is responsible for:

- Resolving `(platform, user_id)` to wallet addresses
- Routing asset transfers to resolved recipients
- Supporting pending payments when recipients are not yet registered
- Integrating with wallet and registry contracts

---

## Features

### Identity-Based Payments

- Send assets using social identifiers
- Automatically resolves recipient wallet via registry
- Supports multiple platforms (e.g. X, email, Telegram)

### Payment Routing

- Transfers assets to resolved wallet addresses
- Uses wallet/token contracts for execution
- Ensures deterministic routing behavior

### Pending Payments

- Stores payments when recipient is not yet registered
- Allows later claim or resolution once identity is linked
- Tracks payment status

---

## Core Functions

### `send_to_userid`

Sends a payment using a social identifier.

**Params:**

- `sender: Address`
- `platform: String`
- `user_id: String`
- `asset: Address`
- `amount: i128`

**Behavior:**

- Resolves recipient via registry
- If found → executes transfer
- If not found → stores as pending payment

---

### `claim_payment`

Claims a pending payment after identity is registered.

**Behavior:**

- Verifies caller identity
- Transfers pending assets to wallet
- Updates payment status

---

### `get_payment`

Fetches payment details by ID.

---

## Data Model

### Payment

- `payment_id: BytesN<32>`
- `sender: Address`
- `recipient (platform + user_id)`
- `asset: Address`
- `amount: i128`
- `status: Pending | Completed`

---

## Integration

Works with:

- **Wallet Contract** → executes transfers
- **Identity Registry** → resolves user identities
- **Token Contracts** → asset transfers

---

## Security

- Sender must authorize payments
- Identity resolution is trusted via registry
- Prevents double-claiming of payments
- Deterministic payment ID generation

---

## Notes

- Payments rely on registry availability for resolution
- Pending payments must be claimed after identity binding
- Ensure platform/user_id normalization for consistency

---

## License

MIT
