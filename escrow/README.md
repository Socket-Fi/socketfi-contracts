# Factory Contract

The **Factory Contract** is the core entry point for wallet creation and upgrade governance within the SocketFi ecosystem.

It is responsible for:

- Creating new wallet instances
- Managing core system contract dependencies
- Handling upgrade governance (proposals, voting, execution)

---

## ✨ Features

### 🔐 Wallet Creation

- Permissionless wallet deployment
- Supports passkey-based identity and BLS key attachments
- Emits wallet creation events

### ⚙️ System Configuration

- Stores and manages:
  - Admin address
  - Registry contract
  - Fee manager contract

### 🗳️ Upgrade Governance

- Proposal-based upgrade system
- Multi-voter approval mechanism
- Admin-triggered execution after approval

---

## 🧱 Contract Overview

- **Contract Name:** `FactoryContract`
- **Trait:** `FactoryTrait`
- **Language:** Rust (Soroban SDK)

Source: :contentReference[oaicite:0]{index=0}

---

## 🚀 Initialization

### `__constructor`

Initializes the factory contract.

#### Parameters:

- `admin: Address` — contract administrator
- `registry: Address` — identity/registry contract
- `fee_manager: Address` — fee management contract
- `wasm: BytesN<32>` — initial wallet WASM hash

#### Behavior:

- Can only be called once
- Sets core configuration
- Initializes wallet version
- Adds admin as initial governance voter

---

## 🪪 Wallet Creation

### `create_wallet`

Creates a new wallet instance.

#### Parameters:

- `passkey: BytesN<77>` — wallet identity/passkey
- `bls_keys: Vec<BytesN<96>>` — optional BLS public keys

#### Returns:

- `Address` — newly created wallet address

#### Notes:

- Wallet creation is permissionless at this level
- Additional validation may exist in downstream logic

---

## 📖 Read Methods

### `get_wallet_version`

Returns the current approved wallet WASM hash.

### `get_admin`

Returns the current admin address.

### `get_registry`

Returns the registry contract address.

### `get_fee_manager`

Returns the fee manager contract address.

---

## ⚙️ Admin Functions

All functions require **admin authorization**.

### `update_admin(new_admin: Address)`

Updates the admin address.

### `update_registry(registry: Address)`

Updates the registry contract.

### `update_fee_manager(fee_manager: Address)`

Updates the fee manager contract.

---

## 🗳️ Governance & Upgrades

### `propose_upgrade`

Creates a new upgrade proposal.

#### Parameters:

- `proposal_type: String`
- `new_wasm_hash: BytesN<32>`

---

### `add_voter`

Adds a new governance voter.

#### Parameters:

- `voter: Address`

---

### `cast_vote`

Casts a vote on an active proposal.

#### Parameters:

- `voter: Address`
- `wasm_hash: BytesN<32>`

---

### `apply_upgrade`

Executes a successful upgrade proposal.

#### Returns:

- `BytesN<32>` — applied WASM hash

---

### `cancel_proposal`

Cancels the active upgrade proposal.

---

## 🔐 Security Model

- **Admin**

  - Controls system configuration
  - Manages governance lifecycle
  - Executes upgrades

- **Voters**

  - Participate in upgrade voting
  - Must be explicitly approved

- **Users**
  - Can create wallets permissionlessly

---

## 📡 Events

The contract emits events for key actions:

- `WalletCreationEvent`
- `UpdateAdminEvent`
- `UpdateRegistryEvent`
- `UpdateFeeManagerEvent`
- `AddVoterEvent`

---

## ⚠️ Notes & Considerations

- Initialization is **one-time only**
- Wallet creation is currently **permissionless**
- `proposal_type` uses `String` (consider enum for stricter validation)
- Ensure voter duplication is prevented in storage layer
- Upgrade execution logic is enforced in the upgrade module

---

## 🧪 Integration Notes

- Designed to work with:
  - Wallet contracts (WASM versions)
  - Registry/identity contracts
  - Fee manager contracts
- Governance module handles voting logic and upgrade lifecycle

---

## 📌 Summary

The Factory Contract acts as the **control layer** of the system:

- Deploys wallets
- Manages dependencies
- Coordinates upgrades securely through governance

---

## License

MIT License

---
