use derive_more::From;
use derive_setters::Setters;
use serde_derive::Serialize;

#[derive(Clone, From)]
#[non_exhaustive]
pub enum ExportRequest {
    #[from(skip)]
    Cmc,
    #[from(skip)]
    DexscreenerLatestBlock,
    #[from(skip)]
    DexscreenerAsset(String),
    #[from(skip)]
    DexscreenerPair(String),
    DexscreenerEvents(DexscreenerEventsParams),
}

impl From<&ExportRequest> for ExportRequest {
    fn from(request: &ExportRequest) -> Self { request.clone() }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct DexscreenerEventsParams {
    #[serde(rename = "fromBlock")]
    pub from_block: u32,
    #[serde(rename = "toBlock")]
    pub to_block: u32,
}

impl DexscreenerEventsParams {
    pub fn new(from_block: u32, to_block: u32) -> Self { Self { from_block, to_block } }
}
