mod builder;

use crate::api_client::builder::Builder;
use crate::v1::V1ApiClient;

/// Default base URL for Swap Coffee API v1.
pub const DEFAULT_API_V1_URL: &str = "https://backend.swap.coffee/v1";

/// Swap Coffee service client with a child client for API v1.
#[derive(Clone)]
#[non_exhaustive]
pub struct SwapCoffeeApiClient {
    /// Swap Coffee API v1 execution client.
    pub v1: V1ApiClient,
}

impl SwapCoffeeApiClient {
    /// Start configuring a Swap Coffee client with the default v1 endpoint.
    pub fn builder() -> Builder { Builder::new() }
}
