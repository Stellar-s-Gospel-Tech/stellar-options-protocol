# Build Plan

**Legend:** ✅ done · 🔨 open

---

## Phase 1 — Foundation ✅

| # | Contract / Item | Status |
|---|---|---|
| 1 | `PriceOracle` — initialize, set/get price, set/get IV | ✅ 4 tests passing |
| 2 | `OptionsWriter` — types, storage keys, view functions | ✅ skeleton with acceptance criteria |
| 3 | `OptionsVault` — initialize, share_price view | ✅ 2 tests passing |
| 4 | `Settlement` — initialize, get_oracle view | ✅ 1 test passing |
| 5 | Workspace, CI, docs, CONTRIBUTING | ✅ |

---

## Phase 2 — Core Options Logic 🔨

### OptionsWriter

| Issue | Task | Complexity |
|---|---|---|
| SOP-001 | `write_option` — validate, lock collateral, store option | Medium |
| SOP-002 | `buy_option` — validate open, transfer premium, set Active | Medium |
| SOP-003 | `exercise` — validate active + caller, swap tokens, set Exercised | High |
| SOP-004 | `reclaim` — validate expired, return collateral, set Expired | Medium |

**First milestone:** write → buy → exercise end-to-end (SOP-001 through SOP-003).

### OptionsVault

| Issue | Task | Complexity |
|---|---|---|
| SOP-005 | `deposit` + `withdraw` — share accounting | High |
| SOP-006 | `roll_epoch` — settle previous + write new covered call | High |

### Settlement

| Issue | Task | Complexity |
|---|---|---|
| SOP-009 | `settle` — read oracle, calculate payout, transfer | High |
| SOP-010 | `settle_batch` — batch settlement for multiple options | Medium |

---

## Phase 3 — Hardening 🔨

| Issue | Task | Complexity |
|---|---|---|
| SOP-007 | Automated strike selection using IV from PriceOracle | High |
| SOP-008 | Reflector oracle integration (replace admin-fed prices) | High |
| SOP-011 | Black-Scholes premium calculation (fixed-point integer math) | High |
| SOP-012 | Put option support in OptionsVault (protective put vault) | Medium |
| SOP-013 | Fuzz tests — property-based testing for payout math | Medium |

---

## Phase 4 — Ecosystem 🔨

| Issue | Task | Complexity |
|---|---|---|
| SOP-014 | TypeScript SDK — typed client wrappers | Medium |
| SOP-015 | Deploy scripts — Testnet deployment + verification | Medium |
| SOP-016 | Integration tests — full write → buy → exercise round-trip | Medium |

---

## Open Contributor Issues

| Issue | Scope | Complexity | Blocked by |
|---|---|---|---|
| **SOP-001** | `write_option` — collateral locking | Medium | — |
| **SOP-002** | `buy_option` — premium transfer | Medium | SOP-001 |
| **SOP-003** | `exercise` — physical settlement | High | SOP-002 |
| **SOP-004** | `reclaim` — collateral return after expiry | Medium | SOP-001 |
| **SOP-005** | Vault `deposit` + `withdraw` | High | SOP-001 |
| **SOP-006** | Vault `roll_epoch` | High | SOP-005 |
| **SOP-009** | `Settlement::settle` — cash settlement | High | SOP-001, PriceOracle |

---

## Milestones

| Milestone | Requires | Status |
|---|---|---|
| **M0 — Foundation** | Phase 1 | ✅ |
| **M1 — Write + Buy + Exercise** | SOP-001, 002, 003 | 🔨 |
| **M2 — Full lifecycle** | SOP-004, 009 | 🔨 |
| **M3 — Passive vault** | SOP-005, 006 | 🔨 |
| **M4 — Production-ready** | Phase 3 | 🔨 |
