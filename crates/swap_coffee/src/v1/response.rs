use crate::v1::types::*;
use derive_more::From;
use derive_setters::Setters;
use serde_derive::Deserialize;

#[macro_export]
macro_rules! unwrap_response {
    ($variant:ident, $result:expr) => {
        match $result {
            $crate::v1::V1Response::$variant(inner) => Ok(inner),
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
pub enum V1Response {
    Assets(Vec<Token>),
    Pools(Vec<PoolsResponse>),
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct PoolsResponse {
    pub total_count: i64,
    pub pools: Vec<PoolInfo>,
}
