use crate::v2::types::*;
use derive_more::From;
use serde_derive::Deserialize;

#[macro_export]
macro_rules! unwrap_response {
    ($variant:ident, $result:expr) => {
        match $result {
            $crate::v2::V2Response::$variant(inner) => Ok(inner),
            other => Err($crate::api_clients_core::ApiClientsError::UnexpectedResponse(format!(
                "ApiClientError: expected {}, but got {:?}",
                stringify!($variant),
                other
            ))),
        }
    };
}

#[derive(Deserialize, Debug, Clone, From)]
#[non_exhaustive]
pub enum V2Response {
    Assets(Vec<Asset>),
    Pools(Vec<Pool>),
    PoolsLite(Vec<PoolLite>),
    PoolTrades(Vec<PoolTrade>),
    RoutingPlan(Vec<Vec<RoutingPlanStep>>),
}
