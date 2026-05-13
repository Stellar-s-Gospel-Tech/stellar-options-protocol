#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(OptionsVault, ());
    let client = OptionsVaultClient::new(&env, &id);

    let admin = soroban_sdk::Address::generate(&env);
    let token = soroban_sdk::Address::generate(&env);
    let writer = soroban_sdk::Address::generate(&env);

    client.initialize(&admin, &token, &writer);
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
    let id = env.register(OptionsVault, ());
    let client = OptionsVaultClient::new(&env, &id);
    let admin = soroban_sdk::Address::generate(&env);
    let token = soroban_sdk::Address::generate(&env);
    let writer = soroban_sdk::Address::generate(&env);
    client.initialize(&admin, &token, &writer);
    client.initialize(&admin, &token, &writer);
}

// ── deposit / withdraw (SOP-005) ──────────────────────────────────────────────
// Remove #[should_panic] once SOP-005 is implemented.

#[test]
#[should_panic]
fn test_deposit_issues_shares() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(OptionsVault, ());
    let client = OptionsVaultClient::new(&env, &id);
    let admin = soroban_sdk::Address::generate(&env);
    let token = soroban_sdk::Address::generate(&env);
    let writer = soroban_sdk::Address::generate(&env);
    let depositor = soroban_sdk::Address::generate(&env);
    client.initialize(&admin, &token, &writer);

    // SOP-005: first deposit should issue 1:1 shares
    client.deposit(&depositor, &100_0000000_i128);
    assert_eq!(client.shares_of(&depositor), 100_0000000_i128);
    assert_eq!(client.get_config().total_shares, 100_0000000_i128);
}
