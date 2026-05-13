# Stellar Options Protocol

On-chain options contracts (calls and puts) for Stellar assets, built with Soroban smart contracts.

> **Status:** Active development — v0 core contracts in progress. Contributions welcome.

---

## The Problem

Stellar has fast, cheap transactions and a growing asset ecosystem (XLM, USDC, tokenised real-world assets). But there are no on-chain hedging tools. Holders of Stellar assets cannot:
- Protect against downside risk without selling
- Generate yield on idle holdings via covered calls
- Express directional views with defined risk

This protocol fills that gap.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     User / Frontend                         │
└──────────┬──────────────────────────────┬───────────────────┘
           │                              │
  ┌────────▼────────┐           ┌─────────▼────────┐
  │  OptionsWriter  │           │   OptionsVault   │
  │  (peer-to-peer) │           │  (passive yield) │
  │  write_option   │           │  deposit         │
  │  buy_option     │           │  withdraw        │
  │  exercise       │           │  roll_epoch      │
  │  reclaim        │           └─────────┬────────┘
  └────────┬────────┘                     │ writes options
           │                              │
           └──────────────┬───────────────┘
                          │
              ┌───────────▼───────────┐
              │      Settlement       │
              │  cash settle at expiry│
              └───────────┬───────────┘
                          │ reads price
              ┌───────────▼───────────┐
              │      PriceOracle      │
              │  spot price + IV feed │
              └───────────────────────┘
```

### Contracts

| Contract | Purpose |
|---|---|
| `options-writer` | Core options lifecycle: write → buy → exercise / reclaim |
| `options-vault` | Passive LP vault that systematically writes covered calls |
| `price-oracle` | Admin-fed spot price and implied volatility (v0); Reflector (v1) |
| `settlement` | Cash settlement at expiry using oracle price |

---

## Option Types

**Call option** — right to buy the underlying at the strike price.
- Writer locks underlying as collateral.
- Buyer pays premium, may exercise before expiry to receive underlying.

**Put option** — right to sell the underlying at the strike price.
- Writer locks strike × amount in quote token as collateral.
- Buyer pays premium, may exercise before expiry to deliver underlying and receive quote.

---

## Quickstart

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt

# Build all contracts
cargo build --release --target wasm32-unknown-unknown

# Run tests
cargo test --all
```

---

## Docs

| Document | Description |
|---|---|
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Design decisions, storage layout, contract interactions |
| [docs/PLAN.md](docs/PLAN.md) | Phase-by-phase build plan |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to contribute |

---

## License

MIT
