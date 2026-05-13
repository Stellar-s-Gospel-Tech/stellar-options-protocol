//! # Pricing — premium and fee calculations
//!
//! ## v0 — writer-set premium
//! The writer specifies the premium when calling `create()`. Simple but requires
//! the writer to price correctly. Suitable for peer-to-peer option writing.
//!
//! ## v1 — Black-Scholes approximation (TODO SOP-008)
//! Use the oracle's implied volatility to calculate a fair premium:
//!
//! ```
//! d1 = (ln(S/K) + (σ²/2) * T) / (σ * √T)
//! d2 = d1 - σ * √T
//! Call premium = S * N(d1) - K * N(d2)
//! Put premium  = K * N(-d2) - S * N(-d1)
//! ```
//!
//! Where:
//! - S = spot price (from PriceOracle)
//! - K = strike price
//! - σ = implied volatility (from PriceOracle::get_implied_vol)
//! - T = time to expiry in years (ledgers_remaining / LEDGERS_PER_YEAR)
//! - N = cumulative normal distribution (integer approximation)
//!
//! All values in 7-decimal fixed-point. No floating point in Soroban.
//!
//! # TODO (SOP-008)
//! Implement `calculate_premium(spot, strike, iv_bps, ledgers_to_expiry) -> i128`
//! using the Black-Scholes approximation above.

/// Approximate ledgers per year at ~5-second ledger intervals.
#[allow(dead_code)]
pub const LEDGERS_PER_YEAR: u32 = 6_307_200;

/// Validate that a writer-set premium is reasonable (> 0).
/// In v1 this will be replaced by Black-Scholes validation.
#[allow(dead_code)]
pub fn validate_premium(premium: i128) -> bool {
    premium > 0
}
