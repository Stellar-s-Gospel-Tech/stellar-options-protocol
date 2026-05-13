# Architecture

## Overview

The Stellar Options Protocol is a set of composable Soroban smart contracts for writing, buying, and settling options on Stellar assets.

**Legend:** ✅ implemented · 🔨 open for contributors

---

## Contract Structure

```
contracts/
├── interfaces/         ✅ Shared types: OptionData, OptionKind, OptionStatus
├── price-oracle/       ✅ Spot price + implied volatility feed (4 tests)
├── options/            ✅ Core options lifecycle
│   ├── src/lib.rs      ← contract entry points (create, buy, exercise, settle, reclaim)
│   ├── src/storage.rs  ← storage keys and helpers
│   └── src/pricing.rs  ← fee calculation (v0: writer-set; v1: Black-Scholes TODO SOP-008)
└── liquidity-pool/     ✅ Passive LP pool
    ├── src/lib.rs      ← provide, withdraw, roll (SOP-006/007 open)
    └── src/pool.rs     ← lock/unlock collateral helpers (SOP-006 open)
```

---

## Contract Interactions

```
LP / User
 │
 ├── LiquidityPool.provide(amount)
 │     └── issues shares, tracks total_underlying
 │
 ├── LiquidityPool.roll(strike, premium, expiry)   ← admin only
 │     ├── reclaims previous epoch's option (if unexercised)
 │     └── calls Options.create(pool, Call, ...) → locks pool's underlying
 │
 ├── Options.create(writer, kind, underlying, quote, amount, strike, premium, expiry)
 │     └── locks collateral from writer into contract
 │
 ├── Options.buy(buyer, option_id)
 │     └── transfers premium from buyer to writer
 │
 ├── Options.exercise(option_id)          ← physical settlement (American)
 │     └── swaps underlying ↔ quote between buyer and contract
 │
 ├── Options.settle(option_id, oracle)    ← cash settlement (European, at expiry)
 │     ├── reads PriceOracle.get_price(underlying)
 │     ├── payout = max(0, spot - strike) * amount / 1e7  [call]
 │     └── transfers payout to buyer; remainder to writer
 │
 └── LiquidityPool.withdraw(shares)
       └── burns shares, returns underlying + accrued premiums
```

---

## Key Data Structures

### OptionData (interfaces)

| Field | Type | Description |
|---|---|---|
| `id` | u64 | Monotonic option ID |
| `kind` | OptionKind | Call or Put |
| `status` | OptionStatus | Open → Active → Exercised / Expired |
| `writer` | Address | Collateral provider |
| `buyer` | Option\<Address\> | Set when premium is paid |
| `underlying_token` | Address | Asset being optioned (XLM SAC or SEP-41) |
| `quote_token` | Address | Payment asset (e.g. USDC) |
| `underlying_amount` | i128 | Collateral (7 decimals) |
| `strike_price` | i128 | Strike in quote per underlying (7 decimals) |
| `premium` | i128 | Premium in quote token |
| `expiry_ledger` | u32 | Ledger sequence at expiry |

### PoolConfig (liquidity-pool)

| Field | Type | Description |
|---|---|---|
| `admin` | Address | Can call `roll` |
| `underlying_token` | Address | Single asset the pool holds |
| `options_contract` | Address | Options contract address |
| `current_epoch` | u32 | Epoch counter |
| `total_shares` | i128 | Outstanding LP shares |
| `total_underlying` | i128 | Total underlying in pool |

---

## Settlement Models

**Physical (`Options::exercise`)** — tokens change hands. American-style, callable any time before expiry. Writer and buyer must both hold the relevant tokens.

**Cash (`Options::settle`)** — oracle price at expiry determines intrinsic value. Payout in `quote_token`. No underlying moves. European-style, callable only at/after expiry.

---

## Pricing

**v0 — writer-set premium:** Writer specifies premium in `create()`. Simple, suitable for peer-to-peer writing.

**v1 — Black-Scholes (SOP-008):**
```
d1 = (ln(S/K) + (σ²/2) * T) / (σ * √T)
d2 = d1 - σ * √T
Call = S * N(d1) - K * N(d2)
```
Where σ = implied vol from `PriceOracle::get_implied_vol`, T = ledgers_to_expiry / LEDGERS_PER_YEAR. Integer fixed-point — no floating point in Soroban.

---

## Deployment Order

```
1. interfaces        (library, no deployment)
2. price-oracle      ✅ no deps
3. options           ✅ no deps at deploy time; oracle address passed per call
4. liquidity-pool    depends on options contract address
```
