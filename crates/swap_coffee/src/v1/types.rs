use core::str;

use derive_setters::Setters;
use serde::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct PoolInfo {
    pub pool: Pool,
    pub info: Info,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Pool {
    pub dex: String,
    pub address: String,
    #[serde(rename = "type")]
    pub pool_type: String,
    pub amm_type: String,
    pub tokens: Vec<Token>,
    pub reserves: Vec<f64>,
    pub fees: Fees,
    pub amm_settings: Option<serde_json::Value>,
    pub restrictions: Option<Vec<Restriction>>,
    pub unavailable_until: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Token {
    pub address: Address,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Address {
    pub blockchain: String,
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub decimals: i64,
    pub listed: bool,
    pub verification: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Fees {
    pub average_gas: f64,
    pub divider: Option<i64>,
    pub input: Option<i64>,
    pub output: Option<i64>,
    pub first_token: Option<i64>,
    pub second_token: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Info {
    pub address: String,
    pub tvl_usd: f64,
    pub volume_usd: f64,
    pub fee_usd: f64,
    pub apr: f64,
    pub lp_apr: f64,
    pub boost_apr: f64,
    pub locked_asset_amount: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Restriction {
    pub min_swap_amount: Option<f64>,
    pub max_swap_amount: Option<f64>,
}
