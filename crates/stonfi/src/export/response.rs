//! Raw STON.fi export response variants.
//!
//! Names and fields intentionally mirror the upstream wire contract.
#![allow(missing_docs, reason = "raw response models mirror the upstream API contract")]

use crate::export::types::{CmcPoolStats, DexscreenerAsset, DexscreenerBlock, DexscreenerEvent, DexscreenerPair};
use derive_more::From;
use derive_setters::Setters;
use serde_derive::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug, Clone, From)]
#[non_exhaustive]
pub enum ExportResponse {
    Cmc(CmcResponse),
    DexscreenerLatestBlock(DexscreenerLatestBlockResponse),
    DexscreenerAsset(DexscreenerAssetResponse),
    DexscreenerPair(DexscreenerPairResponse),
    DexscreenerEvents(DexscreenerEventsResponse),
}

pub type CmcResponse = BTreeMap<String, CmcPoolStats>;

#[derive(Deserialize, Debug, Clone, Default, Setters, PartialEq, Eq)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct DexscreenerLatestBlockResponse {
    pub block: DexscreenerBlock,
}

#[derive(Deserialize, Debug, Clone, Default, Setters, PartialEq, Eq)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct DexscreenerEventsResponse {
    pub events: Vec<DexscreenerEvent>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters, PartialEq, Eq)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct DexscreenerAssetResponse {
    pub asset: DexscreenerAsset,
}

#[derive(Deserialize, Debug, Clone, Default, Setters, PartialEq, Eq)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct DexscreenerPairResponse {
    pub pool: DexscreenerPair,
}
