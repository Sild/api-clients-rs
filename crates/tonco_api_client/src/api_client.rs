mod builder;

use crate::api_client::builder::Builder;
use crate::graphql::GraphqlApiClient;

pub static DEFAULT_GRAPHQL_ENDPOINT: &str = "https://indexer.tonco.io";

#[derive(Clone)]
#[non_exhaustive]
pub struct ToncoApiClient {
    pub graphql: GraphqlApiClient,
}

impl ToncoApiClient {
    pub fn builder() -> Builder { Builder::new() }
}
