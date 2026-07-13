use derive_setters::Setters;
use serde::Deserialize;
use serde_derive::Serialize;

pub type TonAddress = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct PaginatedResponse {
    pub page: PageInfo,
    pub result: Vec<PoolInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct PageInfo {
    pub current: i64,
    pub size: i64,
    pub total: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct PoolInfo {
    pub address: TonAddress,
    pub apr_24h: Option<String>,
    pub base_fee: i32,
    pub bin_step: i32,
    pub created_at: String,
    pub dynamic_fee: i32,
    pub dynamic_fee_factor: i32,
    pub fees_24h: Option<String>,
    pub fraud_status: Option<FraudStatus>,
    pub index: i32,
    pub is_active: Option<bool>,
    pub is_tonfest: bool,
    pub is_tonfest_active: Option<bool>,
    pub lp_fee: i32,
    pub lp_tokens_sum: String,
    pub pool_type: PoolType,
    pub price: Option<String>,
    pub price_change_24h: Option<String>,
    pub protocol_fee: i32,
    pub protocol_fee_reduction: i32,
    pub time_decay: i32,
    pub time_filter: i32,
    pub time_start_trade: Option<String>,
    pub token_x_amount: String,
    pub token_y_amount: String,
    pub tokens: TokensPair,
    pub tokens_distribution: Option<Vec<TokensInBin>>,
    pub tvl: Option<String>,
    pub tvl_locked: Option<String>,
    pub tvl_percent_change_24h: Option<String>,
    pub usd_price: Option<String>,
    pub verified: Option<bool>,
    pub virtual_x_amount: Option<String>,
    pub virtual_y_amount: Option<String>,
    pub volume_24h: Option<String>,
    pub volume_percent_change_24h: Option<String>,
    pub whoami: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct TokensPair {
    pub token_x: Option<TokenInfo>,
    pub token_y: Option<TokenInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct TokenInfo {
    pub address: TonAddress,
    pub banner_url: Option<String>,
    pub bidask_exclusive: Option<bool>,
    pub created_at: String,
    pub decimals: i32,
    pub description: Option<String>,
    pub discord: Option<String>,
    pub icon_url: Option<String>,
    pub instagram: Option<String>,
    pub is_meme: bool,
    pub medium: Option<String>,
    pub name: String,
    pub owner: Option<TonAddress>,
    pub price: Option<String>,
    pub symbol: String,
    pub telegram: Option<String>,
    pub tiktok: Option<String>,
    pub total_supply: Option<serde_json::Number>,
    pub twitter: Option<String>,
    pub verified: bool,
    pub website: Option<String>,
    pub youtube: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct TokensInBin {
    pub bin: i32,
    pub tokens: TokenAmounts,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct TokenAmounts {
    pub token_x_amount: String,
    pub token_y_amount: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum FraudStatus {
    Fine,
    PriceMismatch,
    InitialProvideFailed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PoolType {
    #[default]
    Clmm,
    Dlmm,
    MemeClmm,
    MemeDlmm,
    MemeDamm,
    MemeCpmm,
    Damm,
    Cpmm,
}
