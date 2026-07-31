//! Raw DeDust asset-registry response variants.

use crate::assets::types::Asset;
use serde_derive::Deserialize;

/// Extract the expected payload from a DeDust [`AssetsResponse`](crate::assets::AssetsResponse).
///
/// Returns
/// [`ApiClientsError::UnexpectedResponse`](crate::api_clients_core::ApiClientsError::UnexpectedResponse)
/// when the response variant does not match the requested variant name.
#[macro_export]
macro_rules! unwrap_assets_response {
    ($variant:ident, $result:expr) => {
        match $result {
            $crate::assets::AssetsResponse::$variant(inner) => Ok(inner),
            other => Err($crate::api_clients_core::ApiClientsError::UnexpectedResponse(format!(
                "ApiClientError: expected {}, but got {:?}",
                stringify!($variant),
                other
            ))),
        }
    };
}

/// A typed response returned by the DeDust asset-registry client.
#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum AssetsResponse {
    /// Complete DeDust asset metadata registry.
    List(Vec<Asset>),
}
