//! Raw Stonks response variants.
//!
//! Names and fields intentionally mirror the upstream wire contract.
#![allow(missing_docs, reason = "raw response models mirror the upstream API contract")]

use crate::api::types::PublicToken;
use serde::Deserialize;

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

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum Response {
    PublicTokens(Vec<PublicToken>),
    /// One page of raw TON addresses for discovered Virtual Pools.
    VirtualPoolAddresses(Vec<String>),
    /// All raw TON addresses discovered across every Virtual Pool page.
    AllVirtualPoolAddresses(Vec<String>),
}
