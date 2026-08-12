//! Raw DeDust API v4 pool-registry response variants.

use crate::v4::types::{ClassicPool, CpmmPool, StablePool, UranusPool};
use serde_derive::Deserialize;

/// Extract the expected payload from a DeDust [`V4Response`](crate::v4::V4Response).
///
/// Returns
/// [`ApiClientsError::UnexpectedResponse`](crate::api_clients_core::ApiClientsError::UnexpectedResponse)
/// when the response variant does not match the requested variant name.
#[macro_export]
macro_rules! unwrap_v4_response {
    ($variant:ident, $result:expr) => {
        match $result {
            $crate::v4::V4Response::$variant(inner) => Ok(inner),
            other => Err($crate::api_clients_core::ApiClientsError::UnexpectedResponse(format!(
                "ApiClientError: expected {}, but got {:?}",
                stringify!($variant),
                other
            ))),
        }
    };
}

/// A typed response returned for a DeDust API v4 registry request.
#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum V4Response {
    /// Complete Classic (CPMM v1) pool registry.
    AllClassicPools(Vec<ClassicPool>),
    /// Complete Stable-swap pool registry.
    AllStablePools(Vec<StablePool>),
    /// Complete CPMM v2 pool registry.
    AllCpmmPools(Vec<CpmmPool>),
    /// Complete Uranus launchpad pool registry.
    AllUranusPools(Vec<UranusPool>),
}
