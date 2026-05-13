# Architecture

## Overview

The Stellar Options Protocol is a set of composable Soroban smart contracts for writing, buying, and settling options on Stellar assets.

**Legend:** ✅ implemented · 🔨 open for contributors

---

## Contract Structure

```
contracts/
├── options-writer/     ✅ Core options lifecycle (write, buy, exercise, reclaim)
│                       🔨 write_option, buy_option, exercise, reclaim (SOP-001–004)
├── options-vault/      ✅ initialize, share_price view
│                       🔨 deposit, withdraw, roll_epoch (SOP-005–006)
├── price-oracle/       ✅ Fully implemented — spot price + IV feed
└── settlement/         ✅ initialize
                        🔨 settle (SOP-009)
```

---

## Contract Interactions

```
User
 │
 ├── OptionsWriter.write_option(kind, underlying, quote, amount, strike, premium, expiry)
 │     └── transfers collateral from writer into contract
 │
 ├── OptionsWriter.buy_option(option_id)
 │     └── transfers premium from buyer to writer
 │
 ├── OptionsWriter.exercise(option_id)          ← physical settlement
 │     └── swaps underlying ↔ quote between buyer and contract
 │
 ├── Settlement.settle(option_id)               ← cash settlement
 │     ├── reads PriceOracle.get_price(underlying)
 │     ├── calculates payout = max(0, spot - strike) * amount / 1e7
 │     └── transfers payout to buyer; releases remainder to writer
 │
 └── OptionsVault.deposit(amount)
       └── issues shares → vault writes covered calls via OptionsWriter
```

---

## Key Data Structures

### OptionData (options-writer)

| Field | Type | Description |
|---|---|---|
| `id` | u64 | Monotonic option ID |
| `kind` | OptionKind | Call or Put |
| `status` | OptionStatus | Open → Active → Exercised / Expired |
| `writer` | Address | Collateral provider |
| `buyer` | Option\<Address\> | Set when premium is paid |
| `underlying_token` | Address | Asset being optioned (e.g. XLM SAC) |
| `quote_token` | Address | Payment asset (e.g. USDC) |
| `underlying_amount` | i128 | Collateral amount (7 decimals) |
| `strike_price` | i128 | Strike in quote per underlying unit (7 decimals) |
| `premium` | i128 | Premium in quote token |
| `expiry_ledger` | u32 | Ledger sequence at which option expires |

### VaultConfig (options-vault)

| Field | Type | Description |
|---|---|---|
| `admin` | Address | Can call roll_epoch |
| `underlying_token` | Address | Single asset the vault holds |
| `options_writer` | Address | OptionsWriter contract address |
| `current_epoch` | u32 | Epoch counter |
| `total_shares` | i128 | Outstanding LP shares |
| `total_underlying` | i128 | Total underlying in vault |

---

## Settlement Models

### Physical (OptionsWriter::exercise)
Buyer delivers/receives the actual underlying token. Used for American-style options where the buyer wants the asset.

### Cash (Settlement::settle)
At expiry, the oracle price determines intrinsic value. Payout is in quote token. No underlying changes hands. Simpler for buyers — the model used by Lyra, Hegic, and Premia on Ethereum.

---

## Pricing

**v0 — writer-set premium:** The writer manually sets the premium when calling `write_option`. Simple but requires the writer to price correctly.

**v1 — Black-Scholes approximation (planned):**
```
d1 = (ln(S/K) + (r + σ²/2) * T) / (σ * √T)
d2 = d1 - σ * √T
Call = S * N(d1) - K * e^(-rT) * N(d2)
```
Where S = spot, K = strike, σ = implied vol from PriceOracle, T = time to expiry in years.
Integer approximation using fixed-point math — no floating point in Soroban.

---

## Deployment Order

```
1. PriceOracle          (no deps)
2. OptionsWriter        (no deps)
3. Settlement           (depends on PriceOracle + OptionsWriter)
4. OptionsVault         (depends on OptionsWriter)
```
