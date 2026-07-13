mod builder;

use crate::api_client::builder::Builder;
use crate::graphql::GraphqlApiClient;

/// Default Tonco Indexer GraphQL endpoint.
pub static DEFAULT_GRAPHQL_ENDPOINT: &str = "https://indexer.tonco.io";

/// Tonco service client with a low-level GraphQL child client.
#[derive(Clone)]
#[non_exhaustive]
pub struct ToncoApiClient {
    /// Tonco GraphQL execution client.
    pub graphql: GraphqlApiClient,
}

impl ToncoApiClient {
    /// Start configuring a Tonco client with the default GraphQL endpoint.
    pub fn builder() -> Builder { Builder::new() }
}
