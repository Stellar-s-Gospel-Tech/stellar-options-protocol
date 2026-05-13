//! # LiquidityPool — passive liquidity for options writing
//!
//! LPs deposit a single asset and receive pool shares (LP tokens).
//! The pool systematically writes covered options on behalf of LPs,
//! distributing collected premiums as yield.
//!
//! ## How it works
//!
//! 1. LPs call `provide(amount)` — deposit underlying, receive shares.
//! 2. Each epoch, the admin calls `roll(strike, premium, expiry)`:
//!    a. Settles the previous epoch's option (reclaim if unexercised).
//!    b. Writes a new covered call via the options contract.
//! 3. Premiums flow back into the pool, increasing the share price.
//! 4. LPs call `withdraw(shares)` — burn shares, receive underlying + yield.
//!
//! ## Share accounting
//!
//! Share price = total_underlying / total_shares
//!
//! First deposit: shares issued 1:1 with underlying.
//! Subsequent deposits: shares = amount * total_shares / total_underlying
//!
//! This ensures early LPs benefit from accumulated premiums.
//!
//! ## Module layout
//!
//! - `lib.rs`     — contract entry points
//! - `pool.rs`    — provide, withdraw, lock, unlock helpers
//!
//! ## Contributor guide
//!
//! - Implement `provide` and `withdraw` with share accounting (SOP-006).
//! - Implement `roll` — settle previous epoch + write new option (SOP-007).

#![no_std]

mod pool;

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub struct PoolConfig {
    pub admin: Address,
    /// The asset LPs deposit (e.g. XLM SAC address).
    pub underlying_token: Address,
    /// The options contract this pool writes through.
    pub options_contract: Address,
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
pub struct LiquidityPool;

#[contractimpl]
impl LiquidityPool {
    pub fn initialize(env: Env, admin: Address, underlying_token: Address, options_contract: Address) {
        assert!(!env.storage().instance().has(&DataKey::Config), "already initialized");
        env.storage().instance().set(
            &DataKey::Config,
            &PoolConfig {
                admin,
                underlying_token,
                options_contract,
                current_epoch: 0,
                total_shares: 0,
                total_underlying: 0,
            },
        );
    }

    /// Deposit `amount` of underlying and receive pool shares.
    ///
    /// # TODO (SOP-006)
    /// 1. `provider.require_auth()`
    /// 2. Transfer `amount` of `underlying_token` from provider into pool
    /// 3. Calculate shares:
    ///    if total_shares == 0: shares = amount
    ///    else: shares = amount * total_shares / total_underlying
    /// 4. Update config: total_underlying += amount, total_shares += shares
    /// 5. Store `DataKey::Shares(provider) += shares`
    /// 6. Emit a `Provided` event
    pub fn provide(_env: Env, _provider: Address, _amount: i128) {
        todo!("SOP-006: transfer → calculate shares → update accounting → emit Provided")
    }

    /// Burn `shares` and receive pro-rata underlying + accrued yield.
    ///
    /// # TODO (SOP-006)
    /// 1. `provider.require_auth()`
    /// 2. Assert `shares <= DataKey::Shares(provider)`
    /// 3. underlying_out = shares * total_underlying / total_shares
    /// 4. Update config: total_underlying -= underlying_out, total_shares -= shares
    /// 5. Transfer underlying_out to provider
    /// 6. Emit a `Withdrawn` event
    pub fn withdraw(_env: Env, _provider: Address, _shares: i128) {
        todo!("SOP-006: validate shares → calculate underlying_out → transfer → emit Withdrawn")
    }

    /// Roll to a new epoch: settle previous option and write a new covered call.
    ///
    /// # TODO (SOP-007)
    /// 1. `admin.require_auth()`
    /// 2. If current_epoch > 0:
    ///    - Load previous option ID from `DataKey::EpochOptionId(current_epoch - 1)`
    ///    - Call `OptionsContract::reclaim(prev_id)` if option is still Open/Active
    /// 3. Call `OptionsContract::create(pool_address, Call, ...)` with pool as writer
    /// 4. Store new option ID under `DataKey::EpochOptionId(current_epoch)`
    /// 5. Increment current_epoch in config
    pub fn roll(_env: Env, _strike_price: i128, _premium: i128, _expiry_ledger: u32) {
        todo!("SOP-007: settle previous epoch → write new covered call → increment epoch")
    }

    // ── Views ─────────────────────────────────────────────────────────────────

    pub fn get_config(env: Env) -> PoolConfig {
        env.storage().instance().get(&DataKey::Config).expect("not initialized")
    }

    pub fn shares_of(env: Env, account: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Shares(account))
            .unwrap_or(0)
    }

    /// Current share price in underlying units (7 decimals). Starts at 1.0.
    pub fn share_price(env: Env) -> i128 {
        let config: PoolConfig = env.storage().instance().get(&DataKey::Config).expect("not initialized");
        if config.total_shares == 0 {
            return 1_0000000;
        }
        config.total_underlying * 1_0000000 / config.total_shares
    }
}

#[cfg(test)]
mod test;
