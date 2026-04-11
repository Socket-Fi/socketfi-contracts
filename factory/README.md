# SocketFi Factory Contract

The SocketFi Factory contract is responsible for managing wallet deployment and factory-level configuration for the SocketFi smart wallet system.

It acts as the controlled entry point for creating new wallet contracts and maintaining the approved wallet implementation hash used for deployment.

## Responsibilities

The factory contract is responsible for:

- initializing the factory configuration
- storing the factory admin
- storing the identity registry contract address
- storing the latest approved wallet WASM hash
- deploying new wallet contracts
- updating the approved wallet WASM hash
- updating admin and registry addresses
- upgrading the factory contract itself

## Overview

On deployment, the factory is initialized with:

- an `admin` address
- a `registry` address
- an initial wallet `wasm` hash

After initialization, the factory can deploy new wallet contracts using the currently approved wallet WASM hash.

This ensures wallet creation is controlled and uses only an approved implementation.

## Contract Methods

### `__constructor(e, admin, registry, wasm)`

Initializes the factory contract.

Sets:

- admin address
- registry address
- latest wallet WASM hash

This can only succeed once. If the contract is already initialized, it returns:

- `ContractError::AlreadyInitialized`

---

### `create_wallet(e, passkey, bls_keys)`

Deploys a new wallet contract using the currently stored latest wallet WASM hash.

Parameters:

- `passkey: BytesN<77>` — the wallet's initial passkey
- `bls_keys: Vec<BytesN<96>>` — initial BLS public keys for the wallet

Returns:

- the deployed wallet contract address

This function uses the current approved wallet implementation stored in factory state.

---

### `get_latest_version(e)`

Returns the currently approved wallet WASM hash.

Returns:

- `BytesN<32>`

If no version is set, it returns:

- `ContractError::VersionNotFound`

---

### `get_admin(e)`

Returns the current factory admin address.

---

### `get_registry(e)`

Returns the current registry contract address.

---

### `set_latest_wallet(e, wasm)`

Admin-only function.

Updates the latest approved wallet WASM hash used for future wallet deployments.

Parameters:

- `wasm: BytesN<32>`

---

### `update_admin(e, new_admin)`

Admin-only function.

Updates the factory admin address.

Parameters:

- `new_admin: Address`

---

### `update_registry(e, registry)`

Admin-only function.

Updates the registry contract address.

Parameters:

- `registry: Address`

---

### `upgrade(e, new_wasm_hash)`

Admin-only function.

Upgrades the factory contract itself to a new WASM hash.

Parameters:

- `new_wasm_hash: BytesN<32>`

## Access Control

The following functions are restricted to the factory admin:

- `set_latest_wallet`
- `update_admin`
- `update_registry`
- `upgrade`

Admin authentication is enforced through internal access control logic.

## Wallet Deployment Model

Wallets are not deployed from arbitrary user-supplied WASM hashes.

Instead, the factory stores an approved wallet WASM hash and uses that value when deploying new wallets.

This design improves:

- security
- consistency
- auditability
- upgrade control

## Initialization Flow

Recommended deployment flow:

1. deploy the wallet contract code and obtain its WASM hash
2. deploy the identity registry contract
3. deploy the factory contract
4. call the constructor with:
   - admin address
   - registry address
   - wallet WASM hash

After that, wallet creation can begin through the factory.

## Security Notes

- initialization is protected against multiple executions
- admin-only methods require authorization
- wallet deployments use an approved implementation hash stored in factory state
- factory upgrades are restricted to admin

## Intended Role in the SocketFi System

The factory is one part of the wider SocketFi contract architecture:

- `factory` — deploys wallets and manages wallet implementation versions
- `wallet` — user smart wallet contract
- `identity_registry` — manages verified identity and social profile links
- `shared` — shared types, errors, constants, and utilities

## Errors

This contract uses the shared `ContractError` enum from `socketfi-shared`.

Common errors include:

- `AlreadyInitialized`
- `VersionNotFound`

Other shared errors may also be returned depending on internal logic.

## Notes for Auditors

Key audit considerations:

- wallet creation is controlled by a stored approved WASM hash
- administrative mutation paths are explicitly restricted
- constructor-based initialization is protected against reinitialization
- contract upgrade capability is present and admin-gated

## License

MIT License
