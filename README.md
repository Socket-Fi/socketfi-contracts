# SocketFi Smart Contract Workspace

The **SocketFi Smart Contract Workspace** is a modular system built on the Soroban SDK (Stellar), powering a **secure, upgradeable smart wallet ecosystem** with identity integration and fee abstraction.

It consists of multiple contracts working together to enable wallet creation, governance, identity mapping, and gas abstraction.

---

## ✨ Features

### 🔐 Smart Wallet System

- Contract-based wallets for users
- Supports passkey authentication and BLS keys
- Programmable transaction execution

### ⚙️ Modular Architecture

- Separated contracts for:
  - wallet logic
  - fee abstraction
  - identity registry
  - governance
- Shared modules for reusable logic

### ⚡ Fee Abstraction

- Users can interact without handling native gas tokens
- Flexible fee logic handled by the fee manager

### 🗳️ Governance-Based Upgrades

- Proposal-based upgrade system
- Multi-voter approval mechanism
- Admin-triggered execution after approval

---

## 🧱 Workspace Overview

### Members

- `factory` — wallet deployment and system coordination
- `wallet` — user smart wallet implementation
- `fee_manager` — fee abstraction and payment logic
- `identity_registry` — identity and social mapping
- `upgrade` — governance and upgrade lifecycle
- `shared` — common types, errors, and utilities

---

## 🏭 Factory Contract

The **Factory Contract** is the system entry point.

### Responsibilities:

- Deploy wallet instances
- Store system configuration:
  - admin
  - registry
  - fee manager
  - wallet WASM hash
- Coordinate upgrade governance

### Key Functions:

- `__constructor(admin, registry, fee_manager, wasm)`
- `create_wallet(passkey, bls_keys)`
- `get_wallet_version()`
- `get_admin()`
- `get_registry()`
- `get_fee_manager()`
- `update_admin(new_admin)`
- `update_registry(registry)`
- `update_fee_manager(fee_manager)`

### Governance:

- `propose_upgrade(proposal_type, wasm_hash)`
- `add_voter(voter)`
- `cast_vote(voter, wasm_hash)`
- `apply_upgrade()`
- `cancel_proposal()`

---

## 👛 Wallet Contract

The **Wallet Contract** represents a user’s smart wallet.

### Responsibilities:

- Execute transactions
- Validate authentication (BLS keys)
- Support account abstraction logic

### Features:

- Multiple authentication mechanisms
- Extensible and upgradeable logic
- Interaction with external contracts

---

## 💸 Fee Manager Contract

The **Fee Manager Contract** handles fee abstraction.

### Responsibilities:

- Calculate and enforce transaction fees
- Abstract gas payment logic
- Integrate with wallet execution

### Purpose:

- Enable gasless or flexible payment UX
- Decouple fee handling from wallet logic

---

## 🪪 Identity Registry Contract

The **Identity Registry** manages identity and social linking.

### Responsibilities:

- Map social profiles to wallet addresses
- Resolve identities
- Store verified user data

### Use Cases:

- Social payments
- Username-based addressing
- Account recovery

---

## 🗳️ Upgrade Module

The **Upgrade Module** manages governance.

### Responsibilities:

- Create upgrade proposals
- Manage voter list
- Track votes and deadlines
- Execute approved upgrades

### Design:

- Modular and reusable
- Separated from core contracts for flexibility

---

## 🔗 Shared Module

The **Shared Module** provides reusable components.

### Includes:

- `ContractError`
- shared types
- utility functions
- constants

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
  - Create wallets permissionlessly
  - Interact via wallet contracts

---

## 📡 Events

Contracts emit events for:

- wallet creation
- admin updates
- registry updates
- fee manager updates
- voter additions
- governance actions

These events enable:

- frontend updates
- indexing
- analytics

---

## 🚀 Initialization & Deployment

### Deployment Flow:

1. deploy wallet WASM
2. deploy identity registry
3. deploy fee manager
4. deploy factory contract
5. initialize factory with:
   - admin
   - registry
   - fee manager
   - wallet WASM hash

After initialization:

- wallets can be created
- governance controls upgrades

---

## ⚙️ Build Configuration

### Workspace

```toml
[workspace]
members = [
    "factory",
    "fee_manager",
    "wallet",
    "identity_registry",
    "shared",
    "upgrade"
]

resolver = "2"

[workspace.package]
version = "0.1.0"
rust-version = "1.84.0"

[workspace.dependencies]
soroban-sdk = "25.3.1"
```
