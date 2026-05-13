//! # OptionsVault — liquidity pool for covered call writing
//!
//! Inspired by Ribbon Finance (Ethereum) and Friktion (Solana), this contract
//! implements a **Decentralised Options Vault (DOV)**. Passive LPs deposit a
//! single asset; the vault systematically writes covered call options on their
//! behalf and distributes the collected premiums as yield.
//!
//! ## How it works
//!
//! 1. LPs deposit `underlying_token` and receive vault shares (LP tokens).
//! 2. Each epoch (e.g. weekly), the vault admin calls `roll_epoch()` which:
//!    a. Settles the previous epoch's options (reclaim or exercise).
//!    b. Writes new covered calls via `OptionsWriter::write_option()`.
//! 3. Premiums collected flow back into the vault, increasing the share price.
//! 4. LPs withdraw by burning shares to receive their pro-rata underlying + yield.
//!
//! ## v0 scope
//! - Single asset vault (e.g. XLM-only).
//! - Admin-set strike and expiry per epoch.
//! - No automated Black-Scholes pricing (v1).
//!
//! ## Contributor guide
//!
//! Phase 2 tasks:
//! - Implement `deposit()` and `withdraw()` with share accounting (SOP-005).
//! - Implement `roll_epoch()` — settle previous + write new options (SOP-006).
//! - Add automated strike selection using implied volatility from PriceOracle (SOP-007).

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub struct VaultConfig {
    pub admin: Address,
    pub underlying_token: Address,
    pub options_writer: Address,
    pub current_epoch: u32,
    pub total_shares: i128,
    pub total_underlying: i128,
}

#[contracttype]
pub enum DataKey {
    Config,
    Shares(Address),
    EpochOptionId(u32),
}

#[contract]
pub struct OptionsVault;

#[contractimpl]
impl OptionsVault {
    pub fn initialize(env: Env, admin: Address, underlying_token: Address, options_writer: Address) {
        assert!(
            !env.storage().instance().has(&DataKey::Config),
            "already initialized"
        );
        env.storage().instance().set(
            &DataKey::Config,
            &VaultConfig {
                admin,
                underlying_token,
                options_writer,
                current_epoch: 0,
                total_shares: 0,
                total_underlying: 0,
            },
        );
    }

    /// Deposit `amount` of underlying and receive vault shares.
    ///
    /// Share price = total_underlying / total_shares (starts at 1:1).
    ///
    /// # TODO (SOP-005)
    /// 1. `depositor.require_auth()`
    /// 2. Transfer `amount` of `underlying_token` from depositor into vault
    /// 3. Calculate shares to mint:
    ///    if total_shares == 0: shares = amount
    ///    else: shares = amount * total_shares / total_underlying
    /// 4. Update config: total_underlying += amount, total_shares += shares
    /// 5. Store `DataKey::Shares(depositor) += shares`
    pub fn deposit(_env: Env, _depositor: Address, _amount: i128) {
        todo!("SOP-005: transfer → calculate shares → update accounting")
    }

    /// Burn shares and receive pro-rata underlying + accrued yield.
    ///
    /// # TODO (SOP-005)
    /// 1. `withdrawer.require_auth()`
    /// 2. Load shares; assert shares_to_burn <= depositor's balance
    /// 3. underlying_out = shares_to_burn * total_underlying / total_shares
    /// 4. Update config: total_underlying -= underlying_out, total_shares -= shares_to_burn
    /// 5. Transfer underlying_out to withdrawer
    pub fn withdraw(_env: Env, _withdrawer: Address, _shares_to_burn: i128) {
        todo!("SOP-005: validate shares → calculate underlying_out → transfer → update accounting")
    }

    /// Roll to a new epoch: settle previous options and write new ones.
    ///
    /// # TODO (SOP-006)
    /// 1. `admin.require_auth()`
    /// 2. If current_epoch > 0: call `OptionsWriter::reclaim(prev_option_id)` if unexercised
    /// 3. Call `OptionsWriter::write_option(...)` with vault as writer
    /// 4. Store new option ID under `DataKey::EpochOptionId(new_epoch)`
    /// 5. Increment current_epoch
    pub fn roll_epoch(
        _env: Env,
        _strike_price: i128,
        _premium: i128,
        _expiry_ledger: u32,
    ) {
        todo!("SOP-006: settle previous epoch → write new covered call → increment epoch")
    }

    // ── Views ─────────────────────────────────────────────────────────────────

    pub fn get_config(env: Env) -> VaultConfig {
        env.storage().instance().get(&DataKey::Config).expect("not initialized")
    }

    pub fn shares_of(env: Env, account: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Shares(account))
            .unwrap_or(0)
    }

    pub fn share_price(env: Env) -> i128 {
        let config: VaultConfig = env
            .storage()
            .instance()
            .get(&DataKey::Config)
            .expect("not initialized");
        if config.total_shares == 0 {
            return 1_0000000; // 1.0 at 7 decimals
        }
        config.total_underlying * 1_0000000 / config.total_shares
    }
}

#[cfg(test)]
mod test;
