//! # Settlement — cash settlement at expiry
//!
//! Handles **cash settlement** of options at expiry using the oracle price.
//! This is an alternative to physical delivery (handled by OptionsWriter::exercise).
//!
//! ## Physical vs Cash settlement
//!
//! - **Physical** (OptionsWriter): buyer delivers/receives the actual underlying token.
//! - **Cash** (this contract): at expiry, the protocol reads the oracle price and
//!   pays out the intrinsic value in `quote_token` without moving the underlying.
//!
//! Cash settlement is simpler for buyers (no need to hold the underlying) and
//! is the model used by most DeFi options protocols (Lyra, Hegic, Premia).
//!
//! ## Settlement formula
//!
//! Call payout = max(0, spot_price - strike_price) * underlying_amount / 1e7
//! Put payout  = max(0, strike_price - spot_price) * underlying_amount / 1e7
//!
//! ## Contributor guide
//!
//! Phase 2 tasks:
//! - Implement `settle(option_id)` — read oracle, calculate payout, transfer (SOP-009).
//! - Add batch settlement `settle_batch(option_ids)` for gas efficiency (SOP-010).

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Admin,
    Oracle,
    OptionsWriter,
}

#[contract]
pub struct Settlement;

#[contractimpl]
impl Settlement {
    pub fn initialize(env: Env, admin: Address, oracle: Address, options_writer: Address) {
        assert!(!env.storage().instance().has(&DataKey::Admin), "already initialized");
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Oracle, &oracle);
        env.storage().instance().set(&DataKey::OptionsWriter, &options_writer);
    }

    /// Cash-settle an expired option using the oracle spot price.
    ///
    /// Call payout = max(0, spot - strike) * underlying_amount / 1e7
    /// Put payout  = max(0, strike - spot) * underlying_amount / 1e7
    ///
    /// Payout is transferred from the contract's quote_token balance to the buyer.
    /// If OTM (payout == 0), the writer's collateral is released instead.
    ///
    /// # TODO (SOP-009)
    /// 1. Load option from OptionsWriter; assert status == Active
    /// 2. Assert `env.ledger().sequence() >= option.expiry_ledger`
    /// 3. Call `PriceOracle::get_price(option.underlying_token)` for spot price
    /// 4. Calculate payout based on option.kind
    /// 5. If payout > 0: transfer payout of quote_token to buyer
    /// 6. Release remaining collateral to writer
    /// 7. Mark option as Exercised or Expired via OptionsWriter
    pub fn settle(_env: Env, _option_id: u64) {
        todo!("SOP-009: load option → read oracle → calculate payout → transfer → update status")
    }

    // ── Views ─────────────────────────────────────────────────────────────────

    pub fn get_oracle(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Oracle).expect("not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(Settlement, ());
        let client = SettlementClient::new(&env, &id);
        let admin = Address::generate(&env);
        let oracle = Address::generate(&env);
        let writer = Address::generate(&env);
        client.initialize(&admin, &oracle, &writer);
        assert_eq!(client.get_oracle(), oracle);
    }
}
