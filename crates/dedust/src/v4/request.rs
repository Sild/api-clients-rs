//! Raw DeDust API v4 pool-discovery requests and parameters.

use derive_more::From;
use derive_setters::Setters;
use serde_derive::Serialize;

/// A registry request supported by the DeDust API v4 pool-discovery client.
#[derive(Clone, From)]
#[non_exhaustive]
pub enum V4Request {
    /// Load every Classic (CPMM v1) pool descriptor.
    #[from(skip)]
    AllClassicPools,
    /// Load every Stable-swap pool descriptor.
    #[from(skip)]
    AllStablePools,
    /// Load every CPMM v2 pool descriptor and its fee configuration.
    #[from(skip)]
    AllCpmmPools,
    /// Load every Uranus launchpad pool descriptor.
    #[from(skip)]
    AllUranusPools,
}

/// Filters, sorting, and pagination for the enriched pool screener.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct PoolsParams {
    /// Zero-based grouped-row offset.
    pub offset: u32,
    /// Maximum grouped rows to return. The upstream API currently accepts at most 100.
    pub limit: u32,
    /// Raw upstream sort key, such as `tvl`, `volume_24h`, or `apr`.
    pub sort_by: String,
    /// Raw upstream sort direction, currently `asc` or `desc`.
    pub sort_direction: String,
    /// Optional wallet address used for wallet-aware pool data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    /// Asset identifiers whose metadata should be included in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_assets: Option<Vec<String>>,
    /// Pool addresses to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_addresses: Option<Vec<String>>,
    /// Asset identifiers used to filter pool rows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_assets: Option<Vec<String>>,
    /// Raw pool types to include, such as `cpmm_v1`, `cpmm_v2`, or `stable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_type: Option<Vec<String>>,
    /// Raw DeDust pool tags to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl PoolsParams {
    /// Create parameters using the largest upstream page size and the web application's sort defaults.
    #[must_use]
    pub fn new() -> Self { Self::default() }
}

impl Default for PoolsParams {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 100,
            sort_by: "volume_24h".to_owned(),
            sort_direction: "desc".to_owned(),
            wallet_address: None,
            include_assets: None,
            pool_addresses: None,
            filter_by_assets: None,
            filter_by_type: None,
            tags: None,
        }
    }
}

impl From<&V4Request> for V4Request {
    fn from(request: &V4Request) -> Self { request.clone() }
}
