mod builder;

use crate::api::ApiClient;
use crate::api_client::builder::Builder;

pub const DEFAULT_API_URL: &str = "https://api.bidask.finance/api";

#[derive(Clone)]
#[non_exhaustive]
pub struct BidaskApiClient {
    pub api: ApiClient,
}

impl BidaskApiClient {
    pub fn builder() -> Builder { Builder::new() }
}
