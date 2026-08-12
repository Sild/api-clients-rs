mod request;
mod response;
mod types;

use api_clients_core::{ApiClientsResult, Executor};
use std::sync::Arc;

pub use request::*;
pub use response::*;
pub use types::*;

const JSON_CONTENT_TYPE: (&str, &str) = ("content-type", "application/json");

/// Executes typed requests against the DeDust API v4 pool-discovery service.
#[derive(Clone)]
pub struct V4ApiClient {
    executor: Arc<Executor>,
}

impl V4ApiClient {
    pub(crate) fn new(executor: Arc<Executor>) -> Self { Self { executor } }

    /// Load a filtered page of enriched pool rows used by the DeDust web application.
    ///
    /// # Errors
    ///
    /// Returns an error when transport, status handling, request serialization, or
    /// response deserialization fails.
    pub async fn pools(&self, params: &PoolsParams) -> ApiClientsResult<PoolsResponse> {
        let headers = [(JSON_CONTENT_TYPE.0.to_owned(), JSON_CONTENT_TYPE.1.to_owned())];
        self.executor.exec_post_body("get_pools", params, &headers).await
    }

    /// Execute a DeDust v4 registry request and return its matching response variant.
    ///
    /// # Errors
    ///
    /// Returns an error when transport, status handling, or response deserialization fails.
    pub async fn exec<REQUEST>(&self, request: REQUEST) -> ApiClientsResult<V4Response>
    where
        REQUEST: Into<V4Request>,
    {
        let response = match request.into() {
            V4Request::AllClassicPools => {
                V4Response::AllClassicPools(self.executor.exec_get("get_pools_allclassic").await?)
            }
            V4Request::AllStablePools => {
                V4Response::AllStablePools(self.executor.exec_get("get_pools_allstable").await?)
            }
            V4Request::AllCpmmPools => V4Response::AllCpmmPools(self.executor.exec_get("get_pools_allcpmm").await?),
            V4Request::AllUranusPools => {
                V4Response::AllUranusPools(self.executor.exec_get("get_pools_alluranus").await?)
            }
        };
        Ok(response)
    }
}
