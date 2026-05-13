# Build Plan

**Legend:** ✅ done · 🔨 open

---

## Phase 1 — Foundation ✅

| # | Item | Status |
|---|---|---|
| 1 | `interfaces` — shared types (OptionData, OptionKind, OptionStatus) | ✅ |
| 2 | `price-oracle` — spot price + IV feed | ✅ 4 tests |
| 3 | `options` — types, storage, pricing module, view functions | ✅ skeleton |
| 4 | `liquidity-pool` — initialize, share_price view | ✅ 3 tests |
| 5 | Workspace, CI, docs, CONTRIBUTING | ✅ |

---

## Phase 2 — Core Options Logic 🔨

### options contract

| Issue | Task | Complexity |
|---|---|---|
| SOP-001 | `create` — validate, lock collateral, store option, emit event | Medium |
| SOP-002 | `buy` — validate Open, transfer premium to writer, set Active | Medium |
| SOP-003 | `exercise` — physical settlement: validate Active + caller, swap tokens | High |
| SOP-004 | `settle` — cash settlement: read oracle, calculate payout, transfer | High |
| SOP-005 | `reclaim` — return collateral to writer after expiry | Medium |

**First milestone:** create → buy → exercise end-to-end (SOP-001 through SOP-003).

### liquidity-pool contract

| Issue | Task | Complexity |
|---|---|---|
| SOP-006 | `provide` + `withdraw` — share accounting, lock/unlock collateral | High |
| SOP-007 | `roll` — settle previous epoch + write new covered call | High |

---

## Phase 3 — Hardening 🔨

| Issue | Task | Complexity |
|---|---|---|
| SOP-008 | Black-Scholes premium calculation in `pricing.rs` (fixed-point) | High |
| SOP-009 | Reflector oracle integration in `price-oracle` (replace admin-fed) | High |
| SOP-010 | Put option support in `liquidity-pool` (protective put vault) | Medium |
| SOP-011 | Fuzz tests — property-based testing for payout math | Medium |
| SOP-012 | Locked vs free collateral tracking in `pool.rs` | Medium |

---

## Phase 4 — Ecosystem 🔨

| Issue | Task | Complexity |
|---|---|---|
| SOP-013 | TypeScript SDK — typed client wrappers for all contracts | Medium |
| SOP-014 | Deploy scripts — Testnet deployment + address registration | Medium |
| SOP-015 | Integration tests — full create → buy → exercise round-trip | Medium |

---

## Open Contributor Issues

| Issue | Scope | Complexity | Blocked by |
|---|---|---|---|
| **SOP-001** | `Options::create` — collateral locking | Medium | — |
| **SOP-002** | `Options::buy` — premium transfer | Medium | SOP-001 |
| **SOP-003** | `Options::exercise` — physical settlement | High | SOP-002 |
| **SOP-004** | `Options::settle` — cash settlement | High | SOP-001, price-oracle |
| **SOP-005** | `Options::reclaim` — collateral return | Medium | SOP-001 |
| **SOP-006** | `LiquidityPool::provide` + `withdraw` | High | SOP-001 |
| **SOP-007** | `LiquidityPool::roll` | High | SOP-006 |

---

## Milestones

| Milestone | Requires | Status |
|---|---|---|
| **M0 — Foundation** | Phase 1 | ✅ |
| **M1 — Create + Buy + Exercise** | SOP-001, 002, 003 | 🔨 |
| **M2 — Full lifecycle** | SOP-004, 005 | 🔨 |
| **M3 — Passive pool** | SOP-006, 007 | 🔨 |
| **M4 — Production-ready** | Phase 3 | 🔨 |
