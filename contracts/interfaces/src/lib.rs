//! # Shared types and interfaces
//!
//! Defines the core data structures shared across all contracts in the protocol.
//! Keeping types in a single crate ensures the options contract and liquidity
//! pool contract always agree on the shape of `OptionData`.

#![no_std]

use soroban_sdk::{contracttype, Address};

/// Whether the option gives the right to buy (Call) or sell (Put).
#[contracttype]
#[derive(Clone, PartialEq)]
pub enum OptionKind {
    Call,
    Put,
}

/// Lifecycle state of a single option contract.
#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum OptionStatus {
    /// Collateral locked; no buyer yet.
    Open,
    /// Premium paid; option is live and exercisable.
    Active,
    /// Buyer exercised before expiry.
    Exercised,
    /// Expiry passed without exercise; collateral returned to writer.
    Expired,
}

/// All state for a single option contract.
/// Stored in the options contract keyed by `option_id`.
#[contracttype]
#[derive(Clone)]
pub struct OptionData {
    pub id: u64,
    pub kind: OptionKind,
    pub status: OptionStatus,
    pub writer: Address,
    pub buyer: Option<Address>,
    /// The asset being optioned (XLM SAC address or any SEP-41 token).
    pub underlying_token: Address,
    /// The asset used for premium and strike payment (e.g. USDC).
    pub quote_token: Address,
    /// Amount of underlying locked as collateral (7-decimal Stellar units).
    pub underlying_amount: i128,
    /// Strike price in quote_token per underlying unit (7 decimals).
    pub strike_price: i128,
    /// Premium in quote_token the buyer must pay.
    pub premium: i128,
    /// Ledger sequence number after which the option expires.
    pub expiry_ledger: u32,
}
