//! Raw DeDust API v4 pool-discovery wire types.

use derive_setters::Setters;
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A page returned by the enriched pool screener.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct PoolsResponse {
    /// Asset metadata keyed by raw DeDust asset identifier.
    pub assets_metadata: BTreeMap<String, PoolAssetMetadata>,
    /// Grouped asset-pair rows in the requested order.
    pub pool_rows: Vec<PoolRow>,
    /// Total grouped rows matching the request, before pagination.
    pub total_count: u32,
}

/// Aggregated pool data for one ordered asset pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct PoolRow {
    /// Total annual percentage rate represented as a decimal string.
    pub apr: String,
    /// Fee-derived annual percentage rate represented as a decimal string.
    pub apr_fees: String,
    /// Reward-derived annual percentage rate represented as a decimal string.
    pub apr_rewards: String,
    /// Ordered raw asset identifiers shared by the row's pools.
    pub assets: Vec<String>,
    /// Fees accrued in the last 24 hours in USD, represented as a decimal string.
    pub fees_24h_usd: String,
    /// Enriched pools grouped into this asset-pair row.
    pub pools: Vec<EnrichedPool>,
    /// Maximum number of nested pools returned in this row.
    pub pools_limit: u32,
    /// Offset used for the nested pools in this row.
    pub pools_offset: u32,
    /// Total nested pools available for this row.
    pub total_pools: u32,
    /// Total value locked in USD, represented as a decimal string.
    pub tvl_usd: String,
    /// Trading volume in the last 24 hours in USD, represented as a decimal string.
    pub volume_24h_usd: String,
}

/// Enriched pool record returned inside a pool row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct EnrichedPool {
    /// Raw `workchain:hex_hash` pool address.
    pub address: String,
    /// Total annual percentage rate represented as a decimal string.
    pub apr: String,
    /// Fee-derived annual percentage rate represented as a decimal string.
    pub apr_fees: String,
    /// Reward-derived annual percentage rate represented as a decimal string.
    pub apr_rewards: String,
    /// Ordered raw asset identifiers.
    pub assets: Vec<String>,
    /// Creator fee in basis points, represented as a decimal string.
    pub creator_fee: String,
    /// Raw DeDust implementation identifier.
    pub dex: String,
    /// Assets for which the pool collects trading fees.
    pub fee_in_assets: Vec<String>,
    /// Accumulated fees in raw asset units.
    pub fees: Vec<String>,
    /// Fees accrued in the last 24 hours in USD, represented as a decimal string.
    pub fees_24h_usd: String,
    /// Timestamp of the last pool activity, when available.
    pub last_activity_at: Option<String>,
    /// Locked liquidity in raw liquidity-token units, when supplied.
    pub locked_liquidity: Option<String>,
    /// Liquidity-provider fee in basis points, represented as a decimal string.
    pub lp_fee: String,
    /// Protocol fee in basis points, represented as a decimal string.
    pub protocol_fee: String,
    /// Current reserves in raw asset units.
    pub reserves: Vec<String>,
    /// Assets used by active pool rewards, when supplied.
    pub reward_assets: Option<Vec<String>>,
    /// Active reward programs, when supplied.
    pub rewards: Option<Vec<PoolReward>>,
    /// Total liquidity-token supply in raw units.
    pub total_supply: String,
    /// Total trade fee in basis points, represented as a decimal string.
    pub trade_fee: String,
    /// Total value locked in USD, represented as a decimal string.
    pub tvl_usd: String,
    /// Raw pool type, such as `cpmm_v1`, `cpmm_v2`, or `stable`.
    #[serde(rename = "type")]
    pub pool_type: String,
    /// Whether DeDust marks the pool as verified.
    pub verified: bool,
    /// Trading volume in raw asset units.
    pub volume: Vec<String>,
    /// Trading volume in the last 24 hours in USD, represented as a decimal string.
    pub volume_24h_usd: String,
}

/// Reward program attached to an enriched pool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct PoolReward {
    /// Raw reward-asset identifier.
    pub asset: String,
    /// Reward program end timestamp.
    pub end_date: String,
    /// Remaining reward amount in raw asset units.
    pub remaining_budget: String,
    /// Remaining reward value in USD, represented as a decimal string.
    pub remaining_budget_usd: String,
    /// Upstream reward slot index.
    pub reward_index: u32,
    /// Reward emission per day in raw asset units.
    pub tokens_per_day: String,
    /// Reward emission per day in USD, represented as a decimal string.
    pub tokens_per_day_usd: String,
}

/// Asset metadata embedded in an enriched pool response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct PoolAssetMetadata {
    /// Number of decimal places used by the asset.
    pub decimals: u32,
    /// Display description supplied by DeDust.
    pub description: String,
    /// Absolute asset-image URL.
    pub image_url: String,
    /// Display name supplied by DeDust.
    pub name: String,
    /// Optional social links keyed by upstream platform name.
    pub social_links: Option<BTreeMap<String, String>>,
    /// Display ticker supplied by DeDust.
    pub ticker: String,
    /// Current USD price represented as a decimal string.
    pub usd_price: String,
}

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
