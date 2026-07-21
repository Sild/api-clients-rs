//! Raw Stonks response types.
//!
//! Names and fields intentionally mirror the upstream response schema.
#![allow(missing_docs, reason = "raw wire types mirror the upstream API contract")]

use derive_setters::Setters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Setters)]
#[setters(prefix = "with_")]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PublicToken {
    pub symbol: String,
    pub address: String,
    pub buy_tax: u16,
    pub sell_tax: u16,
}
