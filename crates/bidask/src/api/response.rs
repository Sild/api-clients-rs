use derive_more::From;
use serde::Deserialize;

use crate::api::types::*;

#[macro_export]
macro_rules! unwrap_response {
    ($variant:ident, $result:expr) => {
        match $result {
            $crate::api::Response::$variant(inner) => Ok(inner),
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
pub enum Response {
    Pools(Vec<PoolInfo>),
}
