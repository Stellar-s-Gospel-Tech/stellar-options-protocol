//! Pool accounting helpers.
//!
//! # TODO (SOP-006)
//! Implement `lock_collateral` and `unlock_collateral` helpers used by `roll`:
//!
//! - `lock_collateral(env, amount)` — move `amount` from pool's free balance
//!   into a locked sub-account so it cannot be withdrawn while an option is live.
//! - `unlock_collateral(env, amount)` — reverse of lock; called when an option
//!   expires or is exercised and the pool receives back its collateral.
//!
//! Tracking locked vs free collateral prevents LPs from withdrawing funds that
//! are currently backing an active option.
