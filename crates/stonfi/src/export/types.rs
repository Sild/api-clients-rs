use derive_setters::Setters;
use serde_derive::Deserialize;

// CMC export models.
#[derive(Deserialize, Debug, Clone, Default, Setters, PartialEq)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct CmcPoolStats {
    pub base_id: String,
    pub base_liquidity: String,
    pub base_name: String,
    pub base_symbol: String,
    pub base_volume: String,
    pub last_price: String,
    pub quote_id: String,
    pub quote_liquidity: String,
    pub quote_name: String,
    pub quote_symbol: String,
    pub quote_volume: String,
    pub url: String,
}

// Dexscreener export models.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "eventType", rename_all = "camelCase")]
#[non_exhaustive]
pub enum DexscreenerEvent {
    #[serde(rename = "join")]
    Join {
        block: DexscreenerBlock,
        #[serde(rename = "txnId")]
        txn_id: String,
        #[serde(rename = "txnIndex")]
        txn_index: i64,
        #[serde(rename = "eventIndex")]
        event_index: u64,
        maker: String,
        #[serde(rename = "pairId")]
        pair_id: String,
        amount0: String,
        amount1: String,
        reserves: Option<DexscreenerReserves>,
    },
    #[serde(rename = "exit")]
    Exit {
        block: DexscreenerBlock,
        #[serde(rename = "txnId")]
        txn_id: String,
        #[serde(rename = "txnIndex")]
        txn_index: i64,
        #[serde(rename = "eventIndex")]
        event_index: u64,
        maker: String,
        #[serde(rename = "pairId")]
        pair_id: String,
        amount0: String,
        amount1: String,
        reserves: Option<DexscreenerReserves>,
    },
    #[serde(rename = "swap")]
    Swap {
        block: DexscreenerBlock,
        #[serde(rename = "txnId")]
        txn_id: String,
        #[serde(rename = "txnIndex")]
        txn_index: i64,
        #[serde(rename = "eventIndex")]
        event_index: u64,
        maker: String,
        #[serde(rename = "pairId")]
        pair_id: String,
        #[serde(rename = "amount0In")]
        amount0_in: Option<String>,
        #[serde(rename = "amount0Out")]
        amount0_out: Option<String>,
        #[serde(rename = "amount1In")]
        amount1_in: Option<String>,
        #[serde(rename = "amount1Out")]
        amount1_out: Option<String>,
        #[serde(rename = "priceNative")]
        price_native: String,
        reserves: Option<DexscreenerReserves>,
    },
}

#[derive(Deserialize, Debug, Clone, Default, Setters, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct DexscreenerBlock {
    pub block_number: u32,
    pub block_timestamp: i64,
}

#[derive(Deserialize, Debug, Clone, Default, Setters, PartialEq, Eq)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct DexscreenerReserves {
    pub asset0: String,
    pub asset1: String,
}

#[derive(Deserialize, Debug, Clone, Default, Setters, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct DexscreenerAsset {
    pub circulating_supply: Option<String>,
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub total_supply: String,
}

#[derive(Deserialize, Debug, Clone, Default, Setters, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct DexscreenerPair {
    pub asset0_id: String,
    pub asset1_id: String,
    pub created_at_block_number: Option<u32>,
    pub created_at_block_timestamp: Option<i64>,
    pub created_at_txn_id: Option<String>,
    pub fee_bps: Option<u32>,
    pub id: String,
}
