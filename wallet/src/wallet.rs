use soroban_sdk::{
    contract, contractimpl, vec, Address, BytesN, Env, IntoVal, Map, String, Symbol, Val, Vec,
};

use crate::{
    access::{
        is_initialized, read_factory, read_owner, read_web_keys, try_read_owner, write_agg_bls_key,
        write_factory, write_owner,
    },
    constructor::init_constructor,
    invocation_auth::dapp_invoke_auth,
    wallet_bls_auth::{compute_tx_nonce, owner_require_auth, read_nonce},
    wallet_token::{
        read_allowance, read_balance, read_default_spend_limit, read_limit, send_token,
        spend_token, take_token, write_approve, write_default_spend_limit, write_limit,
    },
    wallet_trait::WalletTrait,
};

use socketfi_shared::{
    types::{AccessSettings, WebKeyDetails},
    ContractError,
};

#[contract]
pub struct Wallet;

#[contractimpl]
impl WalletTrait for Wallet {
    //Account initialization, called when the account is created. Set public keys, master contract id and dapp router contract id
    ///Initialize Wallet
    fn __constructor(
        env: Env,
        username: String,
        passkey: BytesN<77>,
        bls_keys: Vec<BytesN<96>>,
        factory: Address,
    ) -> Result<(), ContractError> {
        if is_initialized(&env) {
            return Err(ContractError::AlreadyInitialized);
        }
        init_constructor(env, username, passkey, bls_keys, factory)?;
        Ok(())
    }

    ///Set User's External G Wallet
    fn set_external_wallet(
        env: Env,
        external_wallet: Address,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        let args: Vec<Val> = vec![&env, external_wallet.clone().to_val()];
        let payload = compute_tx_nonce(&env, String::from_str(&env, "set_external_wallet"), args)?;
        owner_require_auth(env.clone(), payload, tx_signature)?;

        write_owner(&env, &external_wallet);
        Ok(())
    }

    ///Updates the withdrawal/approve  limit per transaction
    fn update_default_limit(
        env: Env,
        limit: i128,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        let args: Vec<Val> = vec![&env, limit.clone().into_val(&env)];
        let payload = compute_tx_nonce(&env, String::from_str(&env, "update_default_limit"), args)?;
        owner_require_auth(env.clone(), payload, tx_signature)?;
        if limit < 0 {
            return Err(ContractError::InvalidLimit);
        }
        write_default_spend_limit(&env, limit);
        Ok(())
    }
    ///Set User's External Wallet
    fn reset_account(
        env: Env,
        new_bls_keys: Vec<BytesN<96>>,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        let args: Vec<Val> = vec![&env, new_bls_keys.clone().to_val()];
        let payload = compute_tx_nonce(&env, String::from_str(&env, "reset_account"), args)?;
        owner_require_auth(env.clone(), payload, tx_signature)?;

        write_agg_bls_key(&env, new_bls_keys)?;
        Ok(())
    }
    ///Update Master Wallet Contract
    fn update_factory(
        env: Env,
        factory: Address,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        let args: Vec<Val> = vec![&env, factory.clone().into_val(&env)];
        let payload = compute_tx_nonce(&env, String::from_str(&env, "update_factory"), args)?;
        owner_require_auth(env.clone(), payload, tx_signature)?;
        write_factory(&env, &factory);
        Ok(())
    }

    ///Deposit Tokens
    fn deposit(e: Env, from: Address, token: Address, amount: i128) -> Result<(), ContractError> {
        from.require_auth();
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        take_token(&e, &from, &token, amount);

        Ok(())
    }
    ///Withdraw Tokens
    fn withdraw(
        env: Env,
        to: Address,
        token: Address,
        amount: i128,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        let args: Vec<Val> = vec![
            &env,
            to.clone().into_val(&env),
            token.clone().into_val(&env),
            amount.clone().into_val(&env),
        ];
        let payload = compute_tx_nonce(&env, String::from_str(&env, "withdraw"), args)?;
        owner_require_auth(env.clone(), payload, tx_signature)?;
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        if amount > read_limit(&env, token.clone()) {
            return Err(ContractError::ExceedMaxAllowance);
        }

        send_token(&env, &to, &token, amount);
        
        Ok(())
    }

    fn dapp_invoker(
        env: Env,
        contract_id: Address,
        func: Symbol,
        args: Option<Vec<Val>>,
        auth_vec: Option<Vec<Map<String, Val>>>,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        let a_args: Vec<Val> = vec![
            &env,
            contract_id.clone().into_val(&env),
            func.clone().into_val(&env),
            args.clone().into_val(&env),
            auth_vec.clone().into_val(&env),
        ];
        let payload = compute_tx_nonce(&env, String::from_str(&env, "dapp_invoker"), a_args)?;
        owner_require_auth(env.clone(), payload, tx_signature)?;
        if let Some(p) = auth_vec {
            dapp_invoke_auth(&env, p)?;
        }

        let _res: Val = env.invoke_contract(&contract_id, &func, args.unwrap_or(vec![&env]));

        Ok(())
    }

    ///Add token custom limit
    fn add_limit(
        env: Env,
        token: Address,
        limit: i128,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        let args: Vec<Val> = vec![
            &env,
            token.clone().into_val(&env),
            limit.clone().into_val(&env),
        ];
        let payload = compute_tx_nonce(&env, String::from_str(&env, "add_limit"), args)?;
        owner_require_auth(env.clone(), payload, tx_signature)?;
        if limit < 0 {
            return Err(ContractError::InvalidLimit);
        }

        write_limit(&env, token, limit);
        Ok(())
    }

    ///Approve Spender Allowance
    fn approve(
        env: Env,
        token: Address,
        spender: Address,
        amount: i128,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        let args: Vec<Val> = vec![
            &env,
            token.clone().into_val(&env),
            spender.clone().into_val(&env),
            amount.clone().into_val(&env),
        ];
        let payload = compute_tx_nonce(&env, String::from_str(&env, "approve"), args)?;
        owner_require_auth(env.clone(), payload, tx_signature)?;

        if amount < 0 {
            return Err(ContractError::InvalidAmount);
        }
        if amount > read_limit(&env, token.clone()) {
            return Err(ContractError::ExceedMaxAllowance);
        }

        write_approve(&env, &token, &spender, &amount)?;
        Ok(())
    }

    ///Spend
    fn spend(
        env: Env,
        token: Address,
        spender: Address,
        amount: i128,
        to: Address,
    ) -> Result<(), ContractError> {
        spender.require_auth();
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }
        spend_token(&env, &spender, &token, amount, &to);

        Ok(())
    }

    ///Gets the wallets max allowance and the external access wallet
    fn get_account_parameters(env: Env) -> AccessSettings {
        let default_allowance = read_default_spend_limit(&env);
        let g_account = try_read_owner(&env);
        AccessSettings {
            default_allowance,
            g_account,
        }
    }

    ///Get Passkey
    fn get_passkey(env: Env) -> Result<WebKeyDetails, ContractError> {
        let keys = read_web_keys(&env)?;
        Ok(keys)
    }

    ///Get Spender Allowance
    fn get_allowance(env: Env, token: Address, spender: Address) -> i128 {
        read_allowance(&env, &token, &spender)
    }

    ///Get Current Nonce
    fn get_nonce(env: Env) -> Option<BytesN<32>> {
        read_nonce(&env)
    }
    ///Computes and returns tx payload binding
    fn get_tx_payload(env: Env, func: String, args: Vec<Val>) -> Result<BytesN<32>, ContractError> {
        compute_tx_nonce(&env, func, args)
    }

    ///Get Token Balance
    fn get_balance(env: Env, token: Address) -> i128 {
        read_balance(&env, &token)
    }

    ///Get owners linked external G account
    fn get_owner(env: Env) -> Result<Address, ContractError> {
        let wallet = read_owner(&env)?;
        Ok(wallet)
    }

    ///Get Master Contract
    fn get_factory(env: Env) -> Result<Address, ContractError> {
        let master = read_factory(&env)?;

        Ok(master)
    }

    ///Upgrade Contract
    fn upgrade(
        env: Env,
        wasm: BytesN<32>,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        let args: Vec<Val> = vec![&env, wasm.clone().into_val(&env)];
        let payload = compute_tx_nonce(&env, String::from_str(&env, "upgrade"), args)?;
        owner_require_auth(env.clone(), payload, tx_signature)?;

        env.deployer().update_current_contract_wasm(wasm.clone());

        Ok(())
    }
}
