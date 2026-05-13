//! # Options — core options contract
//!
//! Handles the full lifecycle of individual options contracts on Stellar:
//!
//!   create → buy → exercise (physical) / settle (cash) / expire
//!
//! ## Design — fully-collateralised model
//!
//! The writer locks the full collateral upfront. This eliminates counterparty
//! risk without requiring a liquidation engine, making it the right model for
//! a v0 protocol where simplicity and safety matter more than capital efficiency.
//!
//! ### Call option
//! - Writer locks `underlying_amount` of `underlying_token`.
//! - Buyer pays `premium` in `quote_token`.
//! - Exercise: buyer pays `strike * amount` in `quote_token`, receives underlying.
//!
//! ### Put option
//! - Writer locks `strike * amount` in `quote_token`.
//! - Buyer pays `premium` in `quote_token`.
//! - Exercise: buyer delivers `underlying_amount`, receives locked `quote_token`.
//!
//! ## Settlement modes
//!
//! - **Physical** (`exercise`): tokens actually change hands. American-style —
//!   callable any time before `expiry_ledger`.
//! - **Cash** (`settle`): at expiry, oracle price determines intrinsic value;
//!   payout is in `quote_token`. No underlying moves. European-style.
//!
//! ## Module layout
//!
//! - `lib.rs`     — contract entry points
//! - `storage.rs` — storage keys and helpers
//! - `pricing.rs` — fee/premium calculation (v0: writer-set; v1: Black-Scholes)

#![no_std]
#![allow(clippy::too_many_arguments)]

mod pricing;
mod storage;

use interfaces::{OptionData, OptionKind};
use soroban_sdk::{contract, contractimpl, Address, Env};

pub use interfaces::{OptionData as OptionDataExport, OptionKind as OptionKindExport};

#[contract]
pub struct OptionsContract;

#[contractimpl]
impl OptionsContract {
    /// Create a new option by locking collateral. Returns the option ID.
    ///
    /// **Call**: locks `underlying_amount` of `underlying_token`.
    /// **Put**: locks `strike_price * underlying_amount / 1e7` of `quote_token`.
    ///
    /// # TODO (SOP-001)
    /// 1. `writer.require_auth()`
    /// 2. Validate: `underlying_amount > 0`, `strike_price > 0`, `premium > 0`,
    ///    `expiry_ledger > env.ledger().sequence()`
    /// 3. Transfer collateral from writer into this contract
    /// 4. Assign next ID, store `OptionData` with status `Open`
    /// 5. Emit a `Created` event: `env.events().publish(("option", "created"), id)`
    /// 6. Return the option ID
    pub fn create(
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
        todo!("SOP-001: validate → lock collateral → store → emit Created → return id")
    }

    /// Buy an open option by paying the premium to the writer.
    ///
    /// # TODO (SOP-002)
    /// 1. Load option; assert `status == Open`
    /// 2. Assert `env.ledger().sequence() < expiry_ledger`
    /// 3. `buyer.require_auth()`
    /// 4. Transfer `premium` of `quote_token` from buyer to writer
    /// 5. Set `status = Active`, `buyer = Some(buyer)`; persist
    /// 6. Emit a `Purchased` event
    pub fn buy(_env: Env, _buyer: Address, _option_id: u64) {
        todo!("SOP-002: validate Open + not expired → transfer premium → Active → emit Purchased")
    }

    /// Physically exercise an active option before expiry.
    ///
    /// Call: buyer pays strike in `quote_token`, receives `underlying_token`.
    /// Put:  buyer delivers `underlying_token`, receives strike in `quote_token`.
    ///
    /// # TODO (SOP-003)
    /// 1. Load option; assert `status == Active`, caller == buyer
    /// 2. Assert `env.ledger().sequence() < expiry_ledger`
    /// 3. `buyer.require_auth()`
    /// 4. Execute token swap based on `kind`
    /// 5. Set `status = Exercised`; persist
    /// 6. Emit an `Exercised` event
    pub fn exercise(_env: Env, _option_id: u64) {
        todo!("SOP-003: validate Active + not expired + caller is buyer → swap → Exercised → emit")
    }

    /// Cash-settle an expired option using the oracle spot price.
    ///
    /// Call payout = max(0, spot - strike) * underlying_amount / 1e7
    /// Put payout  = max(0, strike - spot) * underlying_amount / 1e7
    ///
    /// If OTM (payout == 0), collateral is returned to writer automatically.
    ///
    /// # TODO (SOP-004)
    /// 1. Load option; assert `status == Active`
    /// 2. Assert `env.ledger().sequence() >= expiry_ledger`
    /// 3. Cross-contract call: `PriceOracle::get_price(underlying_token)`
    /// 4. Calculate payout; transfer to buyer if > 0
    /// 5. Return remaining collateral to writer
    /// 6. Set `status = Exercised` (ITM) or `Expired` (OTM); persist
    /// 7. Emit a `Settled` event
    pub fn settle(_env: Env, _option_id: u64, _oracle: Address) {
        todo!(
            "SOP-004: validate expired → read oracle → calculate payout → transfer → emit Settled"
        )
    }

    /// Writer reclaims collateral from an Open or Active option after expiry.
    /// Only callable if `settle` was not called (i.e. status is still Open/Active).
    ///
    /// # TODO (SOP-005)
    /// 1. Load option; assert `status == Open || status == Active`
    /// 2. Assert `env.ledger().sequence() >= expiry_ledger`
    /// 3. `writer.require_auth()`
    /// 4. Return collateral to writer
    /// 5. Set `status = Expired`; persist
    pub fn reclaim(_env: Env, _option_id: u64) {
        todo!("SOP-005: validate expired + writer auth → return collateral → Expired")
    }

    // ── Views ─────────────────────────────────────────────────────────────────

    pub fn get_option(env: Env, option_id: u64) -> OptionData {
        storage::get_option(&env, option_id)
    }

    pub fn option_count(env: Env) -> u64 {
        storage::option_count(&env)
    }
}

#[cfg(test)]
mod test;
