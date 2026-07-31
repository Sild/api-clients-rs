mod request;
mod response;
mod types;

use api_clients_core::{ApiClientsResult, Executor};
use std::sync::Arc;

pub use request::*;
pub use response::*;
pub use types::*;

/// Executes typed requests against the DeDust asset registry.
#[derive(Clone)]
pub struct AssetsApiClient {
    executor: Arc<Executor>,
}

impl AssetsApiClient {
    pub(crate) fn new(executor: Arc<Executor>) -> Self { Self { executor } }

    /// Execute a DeDust asset-registry request and return its matching response variant.
    ///
    /// # Errors
    ///
    /// Returns an error when transport, status handling, or response deserialization fails.
    pub async fn exec<REQUEST>(&self, request: REQUEST) -> ApiClientsResult<AssetsResponse>
    where
        REQUEST: Into<AssetsRequest>,
    {
        let response = match request.into() {
            AssetsRequest::List => AssetsResponse::List(self.executor.exec_get("list.json").await?),
        };
        Ok(response)
    }
}
