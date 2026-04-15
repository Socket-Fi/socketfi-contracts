use soroban_sdk::{
    contract, contractimpl, vec, Address, BytesN, Env, IntoVal, String, Symbol, Val, Vec,
};

use crate::{
    contract_trait::SocialPaymentsTrait,
    data::{PaymentResult, PaymentStatus, PendingPayment},
    nonce::{read_payment_nonce, write_payment_nonce},
    payment::{
        append_identity_payment, append_sender_payment, claim_one, generate_payment_id, now,
        read_identity_payment_ids, read_payment, read_sender_payment_ids, refund_one,
        write_payment,
    },
};
use socketfi_access::access::{
    authenticate_admin, has_admin, read_registry, write_admin, write_registry,
};
use socketfi_shared::{
    events,
    tokens::{
        read_is_supported_asset, read_supported_assets, send_asset, take_asset,
        write_is_supported_asset, write_not_supported_asset,
    },
    utils::validate_userid,
    ContractError,
};
use upgrade::{
    cancel_upgrade_proposal, create_upgrade_proposal, execute_upgrade, upgrade_add_voter,
    write_cast_vote,
};

/// -----------------------------------------------------------------------------
/// SocialPayments Contract
/// -----------------------------------------------------------------------------
///
/// Purpose:
/// - Allows users to send assets to a social identity (`platform + user_id`)
///   instead of requiring the recipient wallet address upfront.
/// - If the identity is already linked in the registry, payment is sent directly.
/// - Otherwise, a pending payment is created and can later be claimed by the
///   wallet that proves ownership of that identity.
/// - Also supports refunds, supported-asset configuration, admin management,
///   and upgrade governance.
///
/// Payment model:
/// - Direct path:
///   sender -> registry lookup succeeds -> recipient wallet receives asset
///
/// - Pending path:
///   sender -> registry lookup fails -> payment stored as pending
///   -> rightful owner later claims
///
/// -----------------------------------------------------------------------------
#[contract]
pub struct SocialPayments;

#[contractimpl]
impl SocialPaymentsTrait for SocialPayments {
    // -------------------------------------------------------------------------
    // 1) Initialization
    // -------------------------------------------------------------------------
    //
    // Security notes:
    // - Constructor must only succeed once.
    // - Admin and registry are foundational trust anchors.
    // - Nonce starts at zero and is used in payment id generation.
    fn __constructor(e: Env, admin: Address, registry: Address) -> Result<(), ContractError> {
        if has_admin(&e) {
            return Err(ContractError::AlreadyInitialized);
        }

        write_admin(&e, &admin);
        write_registry(&e, &registry);
        write_payment_nonce(&e, 0);

        Ok(())
    }

    // -------------------------------------------------------------------------
    // 2) User Payment Actions
    // -------------------------------------------------------------------------
    //
    // Flow:
    // 1. Validate supported asset, auth, amount, user id, duration.
    // 2. Query the registry for a wallet bound to (platform, user_id).
    // 3. If wallet exists:
    //      - transfer directly to resolved wallet
    // 4. Otherwise:
    //      - escrow into pending payment state
    //      - store and index by identity and sender
    fn pay_to_social(
        e: Env,
        from: Address,
        platform: String,
        user_id: String,
        asset: Address,
        amount: i128,
        duration: u64,
    ) -> Result<PaymentResult, ContractError> {
        if !read_is_supported_asset(&e, asset.clone()) {
            return Err(ContractError::UnsupportedAsset);
        }

        from.require_auth();

        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        validate_userid(user_id.clone())?;

        if duration == 0 {
            return Err(ContractError::InvalidDuration);
        }

        let args: Vec<Val> = vec![&e, platform.into_val(&e), user_id.into_val(&e)];

        if let Some(to) = e.invoke_contract(
            &read_registry(&e)?,
            &Symbol::new(&e, "get_wallet_by_userid"),
            args,
        ) {
            take_asset(&e, &from, &asset, amount);
            send_asset(&e, &to, &asset, amount);

            return Ok(PaymentResult::Direct(to));
        } else {
            take_asset(&e, &from, &asset, amount);

            let nonce = read_payment_nonce(&e);

            let payment_id = generate_payment_id(
                &e,
                from.clone(),
                asset.clone(),
                amount,
                platform.clone(),
                user_id.clone(),
                nonce,
            );

            let created_at = now(&e);
            let expires_at = created_at
                .checked_add(duration)
                .ok_or(ContractError::InvalidExpiration)?;

            let payment = PendingPayment {
                payment_id: payment_id.clone(),
                sender: from.clone(),
                asset,
                amount,
                platform: platform.clone(),
                user_id: user_id.clone(),
                created_at,
                expires_at,
                status: PaymentStatus::Pending,
                claimed_by: None,
            };

            write_payment(&e, &payment_id.clone(), &payment)?;
            append_identity_payment(&e, platform, user_id, payment_id.clone())?;
            let n = nonce.checked_add(1).ok_or(ContractError::Overflow)?;
            write_payment_nonce(&e, n);
            append_sender_payment(&e, from, payment_id.clone())?;

            Ok(PaymentResult::Pending(payment_id))
        }
    }

    // Claims a single pending payment.
    fn claim_payment(
        e: Env,
        claimer: Address,
        payment_id: BytesN<32>,
    ) -> Result<(), ContractError> {
        claimer.require_auth();
        claim_one(&e, &claimer, &payment_id)
    }

    // Claims multiple pending payments in sequence.
    //
    // Notes:
    // - Entire call fails if any individual claim fails.
    // - Review whether batch atomicity is intended.
    fn claim_payments(
        e: Env,
        claimer: Address,
        payment_ids: Vec<BytesN<32>>,
    ) -> Result<(), ContractError> {
        claimer.require_auth();

        for payment_id in payment_ids.iter() {
            claim_one(&e, &claimer, &payment_id)?;
        }

        Ok(())
    }

    // Refunds a single payment back to its sender where refund rules allow it.
    //
    // Notes:
    // - Sender must authorize.
    // - Refund eligibility validation is delegated to `refund_one`.
    fn refund_payment(
        e: Env,
        sender: Address,
        payment_id: BytesN<32>,
    ) -> Result<(), ContractError> {
        sender.require_auth();
        refund_one(&e, &sender, &payment_id)
    }

    // Refunds multiple payments in sequence.
    //
    // Notes:
    // - Entire call fails if any individual refund fails.
    // - Review whether batch atomicity is intended.
    fn refund_payments(
        e: Env,
        sender: Address,
        payment_ids: Vec<BytesN<32>>,
    ) -> Result<(), ContractError> {
        sender.require_auth();

        for payment_id in payment_ids.iter() {
            refund_one(&e, &sender, &payment_id)?;
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // 3) Read / Query Functions
    // -------------------------------------------------------------------------
    //
    // These functions expose contract state to frontends, indexers, and auditors.
    // They should remain side-effect free.

    // Returns a stored pending payment by id.
    fn get_payment(e: Env, payment_id: BytesN<32>) -> Result<PendingPayment, ContractError> {
        read_payment(&e, &payment_id)
    }

    // Returns total number of created payment entries as tracked by nonce.
    //
    // Note:
    // - This is effectively the next payment nonce / payment count tracker.
    fn get_nonce(e: Env) -> u64 {
        read_payment_nonce(&e)
    }

    // Returns all payment ids associated with a social identity.
    fn get_identity_payments(
        e: Env,
        platform: String,
        user_id: String,
    ) -> Result<Vec<BytesN<32>>, ContractError> {
        read_identity_payment_ids(&e, platform, user_id)
    }

    // Returns all payment ids created by a given sender.
    fn get_sender_payments(e: Env, sender: Address) -> Result<Vec<BytesN<32>>, ContractError> {
        read_sender_payment_ids(&e, sender)
    }

    // Computes currently claimable amount for a specific identity and asset.
    //
    // Inclusion rules:
    // - payment status must be Pending
    // - payment must not be expired
    // - payment asset must match requested asset
    //
    // Notes:
    // - Excludes expired or already processed payments.
    // - Overflow is guarded with checked_add.
    fn get_claimable_total(
        e: Env,
        platform: String,
        user_id: String,
        asset: Address,
    ) -> Result<i128, ContractError> {
        let ids = read_identity_payment_ids(&e, platform, user_id)?;
        let current_time = now(&e);
        let mut total: i128 = 0;

        for payment_id in ids.iter() {
            let payment = read_payment(&e, &payment_id)?;

            if matches!(payment.status, PaymentStatus::Pending)
                && current_time < payment.expires_at
                && payment.asset == asset
            {
                total = total
                    .checked_add(payment.amount)
                    .ok_or(ContractError::InvalidAmount)?;
            }
        }

        Ok(total)
    }

    // Returns all currently supported assets.
    fn get_supported_assets(e: Env) -> Vec<Address> {
        read_supported_assets(&e)
    }

    // -------------------------------------------------------------------------
    // 4) Admin / Config
    // -------------------------------------------------------------------------
    //
    // All functions below are admin-gated and affect configuration.

    // Adds an asset to the supported asset set.
    fn add_supported_asset(e: Env, asset: Address) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        write_is_supported_asset(&e, asset)
    }

    // Removes an asset from the supported asset set.
    fn remove_supported_asset(e: Env, asset: Address) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        write_not_supported_asset(&e, asset)
    }

    // Update contract admin.
    fn set_admin(e: Env, new_admin: Address) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        write_admin(&e, &new_admin);
        Ok(())
    }

    // -------------------------------------------------------------------------
    // 5) Upgrade Governance
    // -------------------------------------------------------------------------
    //
    // Governance flow:
    // - Admin proposes upgrade
    // - Admin may add voters
    // - Voters cast votes
    // - Admin may cancel proposal
    // - Admin applies upgrade after governance conditions are satisfied
    // Executes a previously approved upgrade proposal.
    fn apply_upgrade(e: Env) -> Result<BytesN<32>, ContractError> {
        authenticate_admin(&e)?;
        execute_upgrade(&e)
    }

    /// Creates a new upgrade proposal.
    ///
    /// Authorization:
    /// - Admin only
    fn propose_upgrade(
        e: Env,
        proposal_type: String,
        new_wasm_hash: BytesN<32>,
    ) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        create_upgrade_proposal(&e, proposal_type, &new_wasm_hash)?;
        Ok(())
    }

    /// Adds a governance voter.
    ///
    /// Authorization:
    /// - Admin only
    fn add_voter(e: Env, voter: Address) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        upgrade_add_voter(&e, &voter)?;

        events::AddVoterEvent {
            value: voter.clone(),
        }
        .publish(&e);

        Ok(())
    }

    /// Casts an upgrade vote.
    ///
    /// Authorization:
    /// - Voter must authorize
    fn cast_vote(e: Env, voter: Address, wasm_hash: BytesN<32>) -> Result<(), ContractError> {
        voter.require_auth();
        write_cast_vote(&e, &voter, &wasm_hash)?;
        Ok(())
    }

    /// Cancels the active upgrade proposal.
    ///
    /// Authorization:
    /// - Admin only
    fn cancel_proposal(e: Env) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        cancel_upgrade_proposal(&e)?;
        Ok(())
    }

    /// Performs a direct contract WASM upgrade.
    ///
    /// Authorization:
    /// - Admin only
    ///
    /// - This is the most privileged function in the contract.
    /// - It bypasses proposal execution flow and should be reviewed carefully.
    /// - If governance-only upgrades are desired, this function may weaken that model.
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        e.deployer().update_current_contract_wasm(new_wasm_hash);
        Ok(())
    }
}
