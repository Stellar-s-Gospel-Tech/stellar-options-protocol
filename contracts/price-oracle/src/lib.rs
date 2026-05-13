//! # PriceOracle — asset price and implied volatility feed
//!
//! Provides two data feeds required by the options protocol:
//!
//! 1. **Spot price** — USD price of the underlying asset (7 decimals).
//!    Used by the settlement contract to determine ITM/OTM at expiry.
//!
//! 2. **Implied volatility (IV)** — annualised volatility in basis points.
//!    Used by the vault to calculate Black-Scholes premiums (v1).
//!
//! ## v0 — admin-fed
//! Prices and IV are pushed by a trusted admin. This is the same pattern
//! used by Aave's PriceOracle and the Stellar Lending Protocol in this repo.
//!
//! ## v1 — Reflector integration
//! Replace admin-fed prices with [Reflector](https://reflector.network/),
//! Stellar's native on-chain oracle aggregator. See issue SOP-008.
//!
//! ## Price precision
//! All prices in USD with 7 decimal places (Stellar convention).
//! Example: $0.12 = 1_200_000

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Admin,
    Price(Address),
    ImpliedVol(Address), // annualised IV in basis points (e.g. 8000 = 80%)
}

#[contract]
pub struct PriceOracle;

#[contractimpl]
impl PriceOracle {
    pub fn initialize(env: Env, admin: Address) {
        assert!(
            !env.storage().instance().has(&DataKey::Admin),
            "already initialized"
        );
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Set USD spot price for `asset`. Admin only.
    pub fn set_price(env: Env, asset: Address, price: i128) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("not initialized");
        admin.require_auth();
        assert!(price > 0, "price must be positive");
        env.storage().instance().set(&DataKey::Price(asset), &price);
    }

    /// Get USD spot price for `asset`. Panics if not set.
    pub fn get_price(env: Env, asset: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Price(asset))
            .expect("price not set for asset")
    }

    /// Set implied volatility for `asset` in basis points. Admin only.
    ///
    /// # TODO (SOP-008)
    /// Replace with Reflector oracle integration so IV is derived from
    /// on-chain price history rather than admin-fed values.
    pub fn set_implied_vol(env: Env, asset: Address, iv_bps: u32) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("not initialized");
        admin.require_auth();
        env.storage()
            .instance()
            .set(&DataKey::ImpliedVol(asset), &iv_bps);
    }

    /// Get implied volatility for `asset` in basis points.
    /// Returns 0 if not set (caller should treat as unavailable).
    pub fn get_implied_vol(env: Env, asset: Address) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ImpliedVol(asset))
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn set_and_get_price() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(PriceOracle, ());
        let client = PriceOracleClient::new(&env, &id);
        let admin = Address::generate(&env);
        let asset = Address::generate(&env);
        client.initialize(&admin);
        client.set_price(&asset, &1_200_000); // $0.12
        assert_eq!(client.get_price(&asset), 1_200_000);
    }

    #[test]
    fn set_and_get_implied_vol() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(PriceOracle, ());
        let client = PriceOracleClient::new(&env, &id);
        client.initialize(&Address::generate(&env));
        let asset = Address::generate(&env);
        client.set_implied_vol(&asset, &8000); // 80% IV
        assert_eq!(client.get_implied_vol(&asset), 8000);
    }

    #[test]
    #[should_panic(expected = "price not set for asset")]
    fn get_price_panics_when_not_set() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(PriceOracle, ());
        let client = PriceOracleClient::new(&env, &id);
        client.initialize(&Address::generate(&env));
        client.get_price(&Address::generate(&env));
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn double_initialize_panics() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(PriceOracle, ());
        let client = PriceOracleClient::new(&env, &id);
        let admin = Address::generate(&env);
        client.initialize(&admin);
        client.initialize(&admin);
    }
}
