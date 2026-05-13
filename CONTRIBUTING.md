# Contributing

Thanks for your interest in contributing!

## Quick Start

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt
cargo test --all
```

## How to Contribute

1. Browse open issues — anything labelled `good first issue` is a great starting point.
2. Comment on an issue before starting work to avoid duplication.
3. Open a PR against `develop`, referencing the issue (`Closes #N`).
4. Address review feedback; maintainer merges.

## Code Standards

- `cargo fmt --all` before committing
- `cargo clippy --all-targets -- -D warnings` must pass
- Every new public function needs at least one unit test
- No `unsafe` code

## Commit Style

```
<type>(<scope>): <short description>

Types: feat, fix, docs, test, refactor, chore
Scope: options-writer, options-vault, price-oracle, settlement, sdk
```

## Complexity Labels

| Label | Examples |
|---|---|
| `complexity:trivial` | Doc fix, view function, small refactor |
| `complexity:medium` | New function with tests, storage change |
| `complexity:high` | Cross-contract integration, math implementation |
