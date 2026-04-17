# Fee Manager Contract

The **Fee Manager Contract** handles transaction fee calculation, collection, and deferral within the SocketFi ecosystem.

It enables flexible fee handling across wallets and contracts, supporting both immediate payment and deferred fee models.

---

## Overview

The Fee Manager is responsible for:

- Calculating transaction fees
- Deciding whether to collect or defer fees
- Tracking pending fees per user
- Integrating with wallet and token contracts for fee settlement

---

## Features

### Fee Calculation

- Computes fees based on transaction parameters
- Supports configurable fee logic
- Returns structured fee decisions

### Fee Collection

- Collects fees directly during transactions
- Transfers fees using token contracts
- Ensures accurate accounting

### Deferred Fees

- Allows fees to be deferred instead of immediately paid
- Tracks accumulated pending fees per user
- Enables future settlement

---

## Core Functions

### `quote_transaction_fee`

Returns the fee decision for a transaction.

**Params:**

- `wallet: Address`
- `tx_asset: Address`
- `tx_amount: i128`

**Returns:**

- `FeeDecision`

---

### `apply_transaction_fee`

Applies the fee decision.

**Behavior:**

- If `CollectNow` → transfers fee immediately
- If `Defer` → updates pending fee balance

---

### `get_pending_fee`

Returns the current pending fee for a wallet.

---

## Data Model

### FeeDecision

- `CollectNow`
  - `fee_asset`
  - `total_fee_in_asset`
  - `total_in_base`
- `Defer`
  - `new_pending_fee_usdc`

### Storage

- Pending fees per wallet
- Fee configuration parameters

---

## Integration

Used by:

- Wallet Contract → applies fees during execution
- Payment Router → applies fees on social payments
- Factory / system contracts → shared fee logic

---

## Security

- Fee logic is deterministic and transparent
- Prevents overflow using safe arithmetic
- Ensures fees are only applied once per transaction
- Relies on wallet authorization for asset transfers

---

## Notes

- Deferred fees accumulate and must be settled later
- Fee precision should be handled carefully (high precision recommended)
- Supports flexible fee strategies across different assets

---

## License

MIT
