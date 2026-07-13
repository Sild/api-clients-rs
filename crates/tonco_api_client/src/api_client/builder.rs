use crate::api_client::{ToncoApiClient, DEFAULT_GRAPHQL_ENDPOINT};
use crate::graphql::GraphqlApiClient;
use api_clients_core::{ApiClientsResult, Executor};
use derive_setters::Setters;
use std::sync::Arc;

/// Builder for [`ToncoApiClient`].
#[derive(Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Builder {
    graphql_endpoint: String,
    executor: Option<Arc<Executor>>,
}

impl Builder {
    pub(super) fn new() -> Self {
        Self {
            graphql_endpoint: DEFAULT_GRAPHQL_ENDPOINT.to_string(),
            executor: None,
        }
    }

    /// Build the configured Tonco client.
    ///
    /// # Errors
    ///
    /// Returns an error if the shared executor cannot be constructed.
    pub fn build(self) -> ApiClientsResult<ToncoApiClient> {
        let executor = match self.executor {
            Some(executor) => executor,
            None => Executor::builder(self.graphql_endpoint).build()?.into(),
        };
        let graphql = GraphqlApiClient::new(executor);
        Ok(ToncoApiClient { graphql })
    }
}
