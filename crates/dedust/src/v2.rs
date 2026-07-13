mod request;
mod response;
mod types;

use api_clients_core::{ApiClientsResult, Executor};
use std::sync::Arc;

pub use request::*;
pub use response::*;
pub use types::*;

#[derive(Clone)]
pub struct V2ApiClient {
    executor: Arc<Executor>,
}

impl V2ApiClient {
    pub(crate) fn new(executor: Arc<Executor>) -> Self { Self { executor } }

    pub async fn exec<REQUEST>(&self, request: REQUEST) -> ApiClientsResult<V2Response>
    where
        REQUEST: Into<V2Request>,
    {
        let request = request.into();
        let response = match &request {
            V2Request::Assets => V2Response::Assets(self.executor.exec_get("assets").await?),
            V2Request::Pools => V2Response::Pools(self.executor.exec_get("pools").await?),
            V2Request::PoolsLite => V2Response::PoolsLite(self.executor.exec_get("pools-lite").await?),
            V2Request::PoolTrades(pool_addr) => {
                let path = format!("pools/{}/trades", pool_addr);
                V2Response::PoolTrades(self.executor.exec_get(&path).await?)
            }
            V2Request::RoutingPlan(params) => {
                V2Response::RoutingPlan(self.executor.exec_post_body("routing/plan", params, &[]).await?)
            }
        };
        Ok(response)
    }
}
