mod builder;

use crate::api_client::builder::Builder;
use crate::v2::V2ApiClient;

/// Default base URL for DeDust API v2.
pub const DEFAULT_API_V2_URL: &str = "https://api.dedust.io/v2";

/// DeDust service client with a child client for API v2.
#[derive(Clone)]
#[non_exhaustive]
pub struct DedustApiClient {
    /// DeDust API v2 execution client.
    pub v2: V2ApiClient,
}

impl DedustApiClient {
    /// Start configuring a DeDust client with the default v2 endpoint.
    pub fn builder() -> Builder { Builder::new() }
}
