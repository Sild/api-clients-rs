//! Raw Stonks response types.
//!
//! Names and fields intentionally mirror the upstream response schema.

use derive_setters::Setters;
use serde::{Deserialize, Serialize};

/// Public-token metadata returned by Stonks.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Setters)]
#[setters(prefix = "with_")]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PublicToken {
    /// Token symbol supplied by Stonks.
    pub symbol: String,
    /// Raw TON token address supplied by Stonks.
    pub address: String,
    /// Raw buy-tax percentage; this value is not converted to basis points.
    pub buy_tax: u16,
    /// Raw sell-tax percentage; this value is not converted to basis points.
    pub sell_tax: u16,
}
