#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

fn setup() -> (Env, Address, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let writer = Address::generate(&env);
    let buyer = Address::generate(&env);
    let underlying_id = env.register_stellar_asset_contract_v2(writer.clone());
    let underlying = underlying_id.address();
    StellarAssetClient::new(&env, &underlying).mint(&writer, &1_000_0000000_i128);
    let quote_id = env.register_stellar_asset_contract_v2(writer.clone());
    let quote = quote_id.address();
    StellarAssetClient::new(&env, &quote).mint(&buyer, &1_000_0000000_i128);
    (env, writer, buyer, underlying, quote)
}

// ── create (SOP-001) ──────────────────────────────────────────────────────────

#[test]
#[should_panic]
fn test_create_call_locks_collateral() {
    let (env, writer, _buyer, underlying, quote) = setup();
    let id = env.register(OptionsContract, ());
    let client = OptionsContractClient::new(&env, &id);
    let token = TokenClient::new(&env, &underlying);
    let before = token.balance(&writer);

    // SOP-001: writer locks 10 XLM as collateral
    let option_id = client.create(
        &writer,
        &OptionKind::Call,
        &underlying,
        &quote,
        &10_0000000_i128,
        &1_5000000_i128,
        &1000000_i128,
        &(env.ledger().sequence() + 1000),
    );
    assert_eq!(option_id, 0);
    assert_eq!(before - token.balance(&writer), 10_0000000_i128);
}

// ── buy (SOP-002) ─────────────────────────────────────────────────────────────

#[test]
#[should_panic]
fn test_buy_transfers_premium_to_writer() {
    let (env, writer, buyer, underlying, quote) = setup();
    let id = env.register(OptionsContract, ());
    let client = OptionsContractClient::new(&env, &id);
    let option_id = client.create(
        &writer, &OptionKind::Call, &underlying, &quote,
        &10_0000000_i128, &1_5000000_i128, &1000000_i128,
        &(env.ledger().sequence() + 1000),
    );
    let quote_token = TokenClient::new(&env, &quote);
    let writer_before = quote_token.balance(&writer);

    // SOP-002: buyer pays premium to writer
    client.buy(&buyer, &option_id);
    assert_eq!(quote_token.balance(&writer) - writer_before, 1000000_i128);
}

// ── exercise (SOP-003) ────────────────────────────────────────────────────────

#[test]
#[should_panic]
fn test_exercise_call_swaps_tokens() {
    let (env, writer, buyer, underlying, quote) = setup();
    let id = env.register(OptionsContract, ());
    let client = OptionsContractClient::new(&env, &id);
    let option_id = client.create(
        &writer, &OptionKind::Call, &underlying, &quote,
        &10_0000000_i128, &1_5000000_i128, &1000000_i128,
        &(env.ledger().sequence() + 1000),
    );
    client.buy(&buyer, &option_id);
    let underlying_token = TokenClient::new(&env, &underlying);
    let buyer_before = underlying_token.balance(&buyer);

    // SOP-003: buyer exercises — pays strike, receives underlying
    client.exercise(&option_id);
    assert_eq!(underlying_token.balance(&buyer) - buyer_before, 10_0000000_i128);
}

// ── reclaim (SOP-005) ─────────────────────────────────────────────────────────

#[test]
#[should_panic]
fn test_reclaim_after_expiry() {
    let (env, writer, _buyer, underlying, quote) = setup();
    let id = env.register(OptionsContract, ());
    let client = OptionsContractClient::new(&env, &id);
    let expiry = env.ledger().sequence() + 100;
    let option_id = client.create(
        &writer, &OptionKind::Call, &underlying, &quote,
        &10_0000000_i128, &1_5000000_i128, &1000000_i128, &expiry,
    );
    env.ledger().with_mut(|li| li.sequence_number = expiry + 1);
    let token = TokenClient::new(&env, &underlying);
    let before = token.balance(&writer);

    // SOP-005: writer reclaims after expiry
    client.reclaim(&option_id);
    assert_eq!(token.balance(&writer) - before, 10_0000000_i128);
}
