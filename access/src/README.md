# Access Package

The **Access Package** provides shared access control and configuration helpers for SocketFi contracts.

It centralizes common logic for reading and managing critical contract addresses and admin permissions.

---

## Overview

The access package is responsible for:

- Managing admin access control
- Storing and retrieving core contract dependencies
- Providing reusable read/write helpers for contract configuration

It is used across contracts to ensure consistent and secure access patterns.

---

## Features

### Admin Management

- Check if admin is set
- Read admin address
- Update admin address

### Dependency Access

Provides access to shared contract addresses such as:

- Identity Registry
- Fee Manager
- Other system contracts

---

## Core Functions

### Admin

- `has_admin` → checks if admin is initialized
- `read_admin` → returns admin address
- `write_admin` → sets admin address

---

### Registry

- `read_registry` → returns registry contract address
- `write_registry` → sets registry contract address

---

### Fee Manager

- `read_fee_manager` → returns fee manager address
- `write_fee_manager` → sets fee manager address

---

## Usage

Import in contracts:

```rust
use socketfi_access::access::*;
```
