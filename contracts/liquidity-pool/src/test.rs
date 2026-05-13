#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(LiquidityPool, ());
    let client = LiquidityPoolClient::new(&env, &id);
    let admin = soroban_sdk::Address::generate(&env);
    let token = soroban_sdk::Address::generate(&env);
    let options = soroban_sdk::Address::generate(&env);
    client.initialize(&admin, &token, &options);
    let config = client.get_config();
    assert_eq!(config.admin, admin);
    assert_eq!(config.total_shares, 0);
    assert_eq!(client.share_price(), 1_0000000);
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_double_initialize_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(LiquidityPool, ());
    let client = LiquidityPoolClient::new(&env, &id);
    let admin = soroban_sdk::Address::generate(&env);
    let token = soroban_sdk::Address::generate(&env);
    let options = soroban_sdk::Address::generate(&env);
    client.initialize(&admin, &token, &options);
    client.initialize(&admin, &token, &options);
}

// ── provide / withdraw (SOP-006) ──────────────────────────────────────────────

#[test]
#[should_panic]
fn test_provide_issues_shares_one_to_one() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(LiquidityPool, ());
    let client = LiquidityPoolClient::new(&env, &id);
    let admin = soroban_sdk::Address::generate(&env);
    let token = soroban_sdk::Address::generate(&env);
    let options = soroban_sdk::Address::generate(&env);
    let provider = soroban_sdk::Address::generate(&env);
    client.initialize(&admin, &token, &options);

    // SOP-006: first deposit should issue 1:1 shares
    client.provide(&provider, &100_0000000_i128);
    assert_eq!(client.shares_of(&provider), 100_0000000_i128);
    assert_eq!(client.get_config().total_shares, 100_0000000_i128);
}
