//! Raw DeDust API v4 pool-registry wire types.

use derive_setters::Setters;
use serde_derive::{Deserialize, Serialize};

/// Raw Classic (CPMM v1) pool descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct ClassicPool {
    /// Ordered raw asset identifiers, such as `native` or `jetton:0:<hash>`.
    pub assets: Vec<String>,
    /// Raw `workchain:hex_hash` pool address.
    pub pool_address: String,
}

/// Raw Stable-swap pool descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StablePool {
    /// Ordered raw asset identifiers, such as `native` or `jetton:0:<hash>`.
    pub assets: Vec<String>,
    /// Raw `workchain:hex_hash` pool address.
    pub pool_address: String,
}

/// Raw CPMM v2 pool descriptor and fee configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct CpmmPool {
    /// Ordered raw asset identifiers, such as `native` or `jetton:0:<hash>`.
    pub assets: Vec<String>,
    /// Base trading fee in basis points.
    pub base_fee_bps: u16,
    /// Raw upstream selector identifying which side collects fees.
    pub fee_in: u8,
    /// Raw `workchain:hex_hash` pool address.
    pub pool_address: String,
}

/// Raw Uranus launchpad pool descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct UranusPool {
    /// Base trading fee in basis points.
    pub base_fee_bps: u16,
    /// Raw integer fundraising target represented as a decimal string.
    pub raising_funds: String,
    /// Raw `workchain:hex_hash` token address.
    pub token_address: String,
}
