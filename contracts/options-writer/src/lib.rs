//! # OptionsWriter — core options contract
//!
//! This contract handles the full lifecycle of a single options contract:
//!
//!   write → buy → exercise / expire
//!
//! ## Design (covered options model)
//!
//! Inspired by Hegic (Ethereum) and PsyOptions (Solana), this contract uses a
//! **fully-collateralised** model: the writer locks the underlying asset upfront,
//! eliminating counterparty risk without requiring a liquidation engine.
//!
//! ### Call option flow
//! 1. Writer locks `underlying_amount` of `underlying_token` as collateral.
//! 2. Buyer pays `premium` in `quote_token` to purchase the option.
//! 3. Before `expiry_ledger`, buyer may exercise by paying `strike_price` in
//!    `quote_token` and receiving `underlying_amount` of `underlying_token`.
//! 4. After `expiry_ledger`, writer reclaims the locked collateral if unexercised.
//!
//! ### Put option flow
//! 1. Writer locks `strike_price * underlying_amount` in `quote_token` as collateral.
//! 2. Buyer pays `premium` in `quote_token`.
//! 3. Before `expiry_ledger`, buyer may exercise by delivering `underlying_amount`
//!    of `underlying_token` and receiving the locked `quote_token`.
//! 4. After `expiry_ledger`, writer reclaims locked collateral if unexercised.
//!
//! ## Pricing
//! v0: writer sets the premium manually (peer-to-peer model).
//! v1: Black-Scholes approximation using on-chain volatility from the vault contract.
//!
//! ## Storage
//! Each option is stored as a single `OptionData` struct keyed by `DataKey::Option(id)`.
//! IDs are monotonically incrementing u64 values.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

// ── Types ─────────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum OptionKind {
    Call,
    Put,
}

#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum OptionStatus {
    /// Writer has locked collateral; awaiting a buyer.
    Open,
    /// A buyer has paid the premium; option is live.
    Active,
    /// Buyer exercised before expiry.
    Exercised,
    /// Expiry passed without exercise; writer reclaimed collateral.
    Expired,
}

/// All state for a single options contract.
#[contracttype]
#[derive(Clone)]
pub struct OptionData {
    pub id: u64,
    pub kind: OptionKind,
    pub status: OptionStatus,
    pub writer: Address,
    pub buyer: Option<Address>,
    /// The asset being optioned (e.g. XLM SAC address).
    pub underlying_token: Address,
    /// The asset used for premium and strike payment (e.g. USDC).
    pub quote_token: Address,
    /// Amount of underlying locked as collateral (7-decimal Stellar units).
    pub underlying_amount: i128,
    /// Strike price in quote_token per underlying unit (7 decimals).
    pub strike_price: i128,
    /// Premium in quote_token the buyer must pay.
    pub premium: i128,
    /// Ledger sequence number after which the option expires.
    pub expiry_ledger: u32,
}

#[contracttype]
pub enum DataKey {
    NextId,
    Option(u64),
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct OptionsWriter;

#[contractimpl]
impl OptionsWriter {
    /// Writer creates a new option by locking collateral.
    ///
    /// For a **Call**: locks `underlying_amount` of `underlying_token`.
    /// For a **Put**: locks `strike_price * underlying_amount / 1e7` of `quote_token`.
    ///
    /// Returns the new option ID.
    ///
    /// # TODO (SOP-001)
    /// Implement this function:
    /// 1. `writer.require_auth()`
    /// 2. Validate: `underlying_amount > 0`, `strike_price > 0`, `premium > 0`,
    ///    `expiry_ledger > env.ledger().sequence()`
    /// 3. Transfer collateral from writer into this contract:
    ///    - Call: transfer `underlying_amount` of `underlying_token`
    ///    - Put:  transfer `strike_price * underlying_amount / 1e7` of `quote_token`
    /// 4. Assign next ID, store `OptionData` with status `Open`
    /// 5. Return the option ID
    pub fn write_option(
        _env: Env,
        _writer: Address,
        _kind: OptionKind,
        _underlying_token: Address,
        _quote_token: Address,
        _underlying_amount: i128,
        _strike_price: i128,
        _premium: i128,
        _expiry_ledger: u32,
    ) -> u64 {
        todo!("SOP-001: validate → transfer collateral → store option → return id")
    }

    /// Buyer purchases an open option by paying the premium.
    ///
    /// # TODO (SOP-002)
    /// 1. Load option; assert status == Open
    /// 2. Assert `env.ledger().sequence() < option.expiry_ledger`
    /// 3. `buyer.require_auth()`
    /// 4. Transfer `premium` of `quote_token` from buyer to writer
    /// 5. Set `option.buyer = Some(buyer)`, `option.status = Active`
    /// 6. Persist updated option
    pub fn buy_option(_env: Env, _buyer: Address, _option_id: u64) {
        todo!("SOP-002: validate open + not expired → transfer premium → set Active")
    }

    /// Buyer exercises an active option before expiry.
    ///
    /// Call exercise: buyer pays `strike_price * underlying_amount / 1e7` of
    /// `quote_token` and receives `underlying_amount` of `underlying_token`.
    ///
    /// Put exercise: buyer delivers `underlying_amount` of `underlying_token`
    /// and receives `strike_price * underlying_amount / 1e7` of `quote_token`.
    ///
    /// # TODO (SOP-003)
    /// 1. Load option; assert status == Active and caller == buyer
    /// 2. Assert `env.ledger().sequence() < option.expiry_ledger`
    /// 3. `buyer.require_auth()`
    /// 4. Execute the token swap based on `option.kind`
    /// 5. Set `option.status = Exercised`; persist
    pub fn exercise(_env: Env, _option_id: u64) {
        todo!("SOP-003: validate active + not expired + caller is buyer → swap tokens → Exercised")
    }

    /// Writer reclaims collateral from an expired, unexercised option.
    ///
    /// # TODO (SOP-004)
    /// 1. Load option; assert status == Active or Open
    /// 2. Assert `env.ledger().sequence() >= option.expiry_ledger`
    /// 3. `writer.require_auth()`
    /// 4. Return collateral to writer
    /// 5. Set `option.status = Expired`; persist
    pub fn reclaim(_env: Env, _option_id: u64) {
        todo!("SOP-004: validate expired ledger → return collateral to writer → Expired")
    }

    // ── Views ─────────────────────────────────────────────────────────────────

    pub fn get_option(env: Env, option_id: u64) -> OptionData {
        env.storage()
            .persistent()
            .get(&DataKey::Option(option_id))
            .expect("option not found")
    }

    pub fn option_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::NextId)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
