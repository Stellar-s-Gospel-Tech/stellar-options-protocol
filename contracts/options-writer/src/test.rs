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

    // Underlying token (XLM-like, 7 decimals)
    let underlying_id = env.register_stellar_asset_contract_v2(writer.clone());
    let underlying_addr = underlying_id.address();
    StellarAssetClient::new(&env, &underlying_addr).mint(&writer, &1_000_0000000_i128);

    // Quote token (USDC-like, 7 decimals)
    let quote_id = env.register_stellar_asset_contract_v2(writer.clone());
    let quote_addr = quote_id.address();
    StellarAssetClient::new(&env, &quote_addr).mint(&buyer, &1_000_0000000_i128);

    (env, writer, buyer, underlying_addr, quote_addr)
}

// ── write_option (SOP-001) ────────────────────────────────────────────────────
// Remove #[should_panic] once SOP-001 is implemented.

#[test]
#[should_panic]
fn test_write_call_option_locks_collateral() {
    let (env, writer, _buyer, underlying, quote) = setup();
    let contract_id = env.register(OptionsWriter, ());
    let client = OptionsWriterClient::new(&env, &contract_id);

    let underlying_token = TokenClient::new(&env, &underlying);
    let balance_before = underlying_token.balance(&writer);

    // SOP-001: writer locks 10 XLM as collateral for a call option
    let id = client.write_option(
        &writer,
        &OptionKind::Call,
        &underlying,
        &quote,
        &10_0000000_i128,  // 10 XLM
        &1_5000000_i128,   // strike: 1.50 USDC per XLM
        &1000000_i128,     // premium: 0.10 USDC
        &(env.ledger().sequence() + 1000),
    );

    assert_eq!(id, 0);
    assert_eq!(balance_before - underlying_token.balance(&writer), 10_0000000_i128);
    assert_eq!(client.option_count(), 1);
}

// ── buy_option (SOP-002) ──────────────────────────────────────────────────────
// Remove #[should_panic] once SOP-002 is implemented.

#[test]
#[should_panic]
fn test_buy_option_transfers_premium() {
    let (env, writer, buyer, underlying, quote) = setup();
    let contract_id = env.register(OptionsWriter, ());
    let client = OptionsWriterClient::new(&env, &contract_id);

    let id = client.write_option(
        &writer,
        &OptionKind::Call,
        &underlying,
        &quote,
        &10_0000000_i128,
        &1_5000000_i128,
        &1000000_i128,
        &(env.ledger().sequence() + 1000),
    );

    let quote_token = TokenClient::new(&env, &quote);
    let writer_balance_before = quote_token.balance(&writer);

    // SOP-002: buyer pays premium to writer
    client.buy_option(&buyer, &id);

    assert_eq!(quote_token.balance(&writer) - writer_balance_before, 1000000_i128);
    let opt = client.get_option(&id);
    assert_eq!(opt.status, OptionStatus::Active);
}

// ── exercise (SOP-003) ────────────────────────────────────────────────────────
// Remove #[should_panic] once SOP-003 is implemented.

#[test]
#[should_panic]
fn test_exercise_call_swaps_tokens() {
    let (env, writer, buyer, underlying, quote) = setup();
    let contract_id = env.register(OptionsWriter, ());
    let client = OptionsWriterClient::new(&env, &contract_id);

    let id = client.write_option(
        &writer,
        &OptionKind::Call,
        &underlying,
        &quote,
        &10_0000000_i128,
        &1_5000000_i128,
        &1000000_i128,
        &(env.ledger().sequence() + 1000),
    );
    client.buy_option(&buyer, &id);

    let underlying_token = TokenClient::new(&env, &underlying);
    let buyer_underlying_before = underlying_token.balance(&buyer);

    // SOP-003: buyer exercises — pays strike, receives underlying
    client.exercise(&id);

    assert_eq!(
        underlying_token.balance(&buyer) - buyer_underlying_before,
        10_0000000_i128
    );
    let opt = client.get_option(&id);
    assert_eq!(opt.status, OptionStatus::Exercised);
}

// ── reclaim (SOP-004) ─────────────────────────────────────────────────────────
// Remove #[should_panic] once SOP-004 is implemented.

#[test]
#[should_panic]
fn test_reclaim_after_expiry() {
    let (env, writer, _buyer, underlying, quote) = setup();
    let contract_id = env.register(OptionsWriter, ());
    let client = OptionsWriterClient::new(&env, &contract_id);

    let expiry = env.ledger().sequence() + 100;
    let id = client.write_option(
        &writer,
        &OptionKind::Call,
        &underlying,
        &quote,
        &10_0000000_i128,
        &1_5000000_i128,
        &1000000_i128,
        &expiry,
    );

    // Advance ledger past expiry
    env.ledger().with_mut(|li| li.sequence_number = expiry + 1);

    let underlying_token = TokenClient::new(&env, &underlying);
    let writer_balance_before = underlying_token.balance(&writer);

    // SOP-004: writer reclaims collateral after expiry
    client.reclaim(&id);

    assert_eq!(
        underlying_token.balance(&writer) - writer_balance_before,
        10_0000000_i128
    );
    let opt = client.get_option(&id);
    assert_eq!(opt.status, OptionStatus::Expired);
}
