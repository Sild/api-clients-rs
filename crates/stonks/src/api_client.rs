mod builder;

use crate::api::ApiClient;
use crate::api_client::builder::Builder;

/// Default base URL for Stonks public API requests.
pub const DEFAULT_API_URL: &str = "https://app.stonks.cash";

/// Stonks service client with an unversioned API child client.
#[derive(Clone)]
#[non_exhaustive]
pub struct StonksApiClient {
    /// Stonks API execution client.
    pub api: ApiClient,
}

impl StonksApiClient {
    /// Start configuring a Stonks client with the default endpoint.
    pub fn builder() -> Builder { Builder::new() }
}
