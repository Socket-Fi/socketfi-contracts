# SocketFi Protocol — STRIDE Threat Model

**System:** SocketFi (Soroban Smart Wallet Ecosystem)  
**Scope:** Factory • Wallet • Identity Registry • Social Payments Router • Fee Manager  
**Purpose:** Security Review / SCF Submission / External Audit Preparation

---

## 1. What are we working on?

SocketFi is a modular smart wallet ecosystem built on Soroban (Stellar), enabling seamless Web3 interactions through passkeys, flexible fee abstraction and social identity.

The system consists of multiple interacting smart contracts that collectively provide wallet deployment, execution, identity resolution, payment routing, and fee management.

---

### Main Capabilities

- Smart wallet deployment and execution
- BLS multi-signature with Passkey-based authentication binding
- Social identity registry and id-wallet resolution capabilities
- Fee abstraction (immediate and deferred) mechanism
- Governance-driven contract upgrades where applicable
- Modular, composable contract architecture

---

### Critical Assets

- Wallet balances and token holdings
- Owner authentication material (passkey - BLS binding)
- dApp Interaction
- Nonce state (replay protection)
- Identity mappings (platform, user_id → wallet)
- Pending social payments
- Fee balances and deferred fee state
- Contract dependency addresses
- Upgrade authority and governance state

---

## 2. High Level Data Flow Diagram (DFD)

### Primary Entities

- User / Wallet Owner
- Wallet Contract
- Factory Contract
- Identity Registry
- Social Payments Router
- Fee Manager
- Token Contracts
- External dApps

---

### Primary Flows

- User → Wallet: Signed transaction payloads (BLS / passkey) or set admin wallet
- Wallet → Fee Manager: Fee quote and application/deferment
- Wallet → Token Contracts: Transfers and approvals
- Wallet → External dApps: Contract invocation
- Router → Registry: Identity resolution
- Router → Wallet: Payment routing
- Factory → Wallet: Deployment and configuration
- Factory → Protocol: Upgrade and governance control

---

### Trust Boundaries

| Boundary                    | Description               | Risk                       |
| --------------------------- | ------------------------- | -------------------------- |
| User → Wallet               | Signed execution payloads | Signature forgery / replay |
| Wallet → Fee Manager        | Fee calculation           | Fee abuse / inconsistency  |
| Router → Registry           | Identity resolution       | Misrouting payments        |
| Factory → Contracts         | Deployment & upgrades     | Governance compromise      |
| Wallet → External Contracts | dApp invocation           | Privilege escalation       |

---

## 3. What can go wrong? (STRIDE Analysis)

| Threat Type            | ID  | Description                                        | Impact                                   | Severity | Affected Contracts  |
| ---------------------- | --- | -------------------------------------------------- | ---------------------------------------- | -------- | ------------------- |
| Spoofing               | S1  | Forged or replayed wallet signatures               | Unauthorized fund movement               | Critical | Wallet              |
| Spoofing               | S2  | Malicious identity mapping                         | Misrouted payments                       | Critical | Registry, Router    |
| Spoofing               | S3  | Malicious contract impersonates dependency         | Unauthorized calls or state manipulation | High     | Wallet, Fee Manager |
| Tampering              | T1  | Malicious upgrade logic                            | Protocol takeover                        | Critical | Factory, Wallet     |
| Tampering              | T2  | Fee manipulation                                   | Incorrect deductions                     | High     | Fee Manager, Wallet |
| Repudiation            | R1  | Missing audit logs                                 | Dispute difficulty                       | Medium   | All                 |
| Information Disclosure | I1  | Identity linkage exposure                          | Privacy loss                             | Medium   | Registry, Router    |
| Denial of Service      | D1  | Dependency failure                                 | Transaction blocking                     | High     | Router, Wallet      |
| Denial of Service      | D2  | Large batch inputs exhaust compute                 | Transaction failure                      | Medium   | Router              |
| Elevation of Privilege | E1  | Authorization bypass                               | Full wallet compromise                   | Critical | Wallet              |
| Elevation of Privilege | E2  | dApp invocation triggers privileged external calls | Unauthorized execution                   | Critical | Wallet              |

---

## 4. What are we going to do about it?

### Spoofing Mitigations (S1, S2, S3)

- Bind signatures to contract ID, network ID, function, arguments, and nonce
- Enforce strict nonce-based replay protection
- Validate identity mappings via validator signatures
- Validate contract identities for all cross-contract interactions

---

### Tampering Mitigations (T1, T2)

- Restrict and validate contract upgrades
- Verify WASM hashes before deployment
- Enforce strict registry write permissions
- Validate fee invariants across quote/apply

---

### Repudiation Mitigations (R1)

- Emit events for all critical actions:
  - transfers
  - approvals
  - payments
  - upgrades
- Log nonce and payload hashes

---

### Information Disclosure Mitigations (I1)

- Restrict sensitive getters
- Avoid exposing unnecessary identity metadata
- Minimize publicly accessible linkage data

---

### Denial of Service Mitigations (D1, D2)

- Limit input sizes and vector lengths
- Validate contract dependencies before execution
- Implement safe failure handling
- Bound batch operations

---

### Elevation of Privilege Mitigations (E1, E2)

- Centralize authorization checks
- Strict owner-only execution paths
- Restrict dApp invocation scope
- Validate cross-contract caller identity

---

## 5. Security Invariants

- Wallet funds can only be moved by:
  - Owner OR authorized spender within limits
- Nonce must strictly increase (no replay)
- Identity mappings must be validator-approved
- Fees must remain within configured bounds
- Only authorized governance can upgrade contracts
- Cross-contract calls must not bypass authentication

---

## 6. Example Attack Scenarios

- **Registry Compromise**

  - Attacker alters identity mapping
  - Router resolves to attacker wallet
  - Funds are irreversibly misrouted

- **Fee Manipulation**

  - Malicious fee parameters applied
  - Wallet drained through excessive fees

- **Upgrade Attack**

  - Malicious WASM deployed
  - Full protocol takeover

- **dApp Invocation Abuse**

  - Wallet calls malicious contract
  - Unauthorized external execution occurs

- **Replay Attack**
  - Signed payload reused
  - Duplicate withdrawals executed

---

## 7. Assumptions

- Soroban host enforces correct execution and authorization
- Token contracts behave according to expected standards
- Validator signatures are securely managed off-chain
- Admin and governance keys are securely managed
- Cross-contract calls behave deterministically

---

## 8. Out of Scope

- Frontend and signing UX security
- Off-chain validator infrastructure
- External token contract correctness
- Soroban host and network security

---

## 9. Architecture Overview

### Core Contracts

- Factory → Deploys wallets and manages configuration
- Wallet → Asset custody and execution
- Identity Registry → Maps identities to wallets
- Social Router → Routes payments
- Fee Manager → Handles fee logic

---

### Contract Relationships

- Factory → deploys → Wallet
- Wallet → uses → Fee Manager
- Wallet → uses → Identity Registry
- Router → resolves → Identity Registry
- Router → routes → Wallet
- Router → uses → Fee Manager
- Factory → integrates → Upgrade
- All contracts → use → Shared + Access

---

## 10. Design Principles

- **Modularity** → Single responsibility per contract
- **Security-first** → Strict authentication and validation
- **Determinism** → Predictable execution
- **Composability** → Clean cross-contract interaction

---

## 11. Integration Flow (Example)

1. User creates wallet via Factory
2. Identity is linked in Registry
3. Payment initiated via Router
4. Router resolves recipient wallet
5. Wallet executes transfer
6. Fee Manager applies fee

---

## 12. Residual Risk

- Validator compromise
- Admin / governance key compromise
- Misconfigured upgrades
- Cross-contract dependency failure
- External token contract behavior

---

## 13. Governance & Upgrade Security

- All admin roles must be protected by multisig or equivalent controls
- Upgrade proposals must be separated from execution
- Governance actions must be auditable and event-driven
- Approved WASM hashes must be reproducible and verified

---

## 14. Conclusion

SocketFi is a multi-contract protocol where security depends on both contract correctness and cross-contract trust.

This STRIDE model provides a comprehensive view of:

- Threat surface
- Attack vectors
- Mitigation strategies

---

## 15. Workspace Structure

```bash
/
├── factory
├── wallet
├── identity_registry
├── social_router
├── fee_manager
├── upgrade
├── shared
├── access
└── docs
    └── security
        └── stride.md
```
