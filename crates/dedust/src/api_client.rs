mod builder;

use crate::api_client::builder::Builder;
use crate::v2::V2ApiClient;

pub const DEFAULT_API_V2_URL: &str = "https://api.dedust.io/v2";

#[derive(Clone)]
#[non_exhaustive]
pub struct DedustApiClient {
    pub v2: V2ApiClient,
}

impl DedustApiClient {
    pub fn builder() -> Builder { Builder::new() }
}
