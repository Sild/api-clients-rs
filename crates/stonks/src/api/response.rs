//! Raw Stonks response variants.
//!
//! Names and fields intentionally mirror the upstream wire contract.

use crate::api::types::{JettonMetadata, PublicToken};
use serde::Deserialize;

/// Extract the expected payload from a Stonks [`Response`](crate::api::Response).
///
/// Returns
/// [`ApiClientsError::UnexpectedResponse`](crate::api_clients_core::ApiClientsError::UnexpectedResponse)
/// when the response variant does not match the requested variant name.
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

/// A typed response returned by the unversioned Stonks API client.
#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum Response {
    /// Public-token metadata, including raw buy and sell tax percentages.
    PublicTokens(Vec<PublicToken>),
    /// One page of raw TON addresses for discovered Virtual Pools.
    VirtualPoolAddresses(Vec<String>),
    /// All raw TON addresses discovered across every Virtual Pool page.
    AllVirtualPoolAddresses(Vec<String>),
    /// Jetton metadata returned for the requested Stonks deployment identifiers.
    JettonMetadataBatch(Vec<JettonMetadata>),
}
