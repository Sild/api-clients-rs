#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Re-export of the shared executor and error crate used by this client.
pub use api_clients_core;
/// Top-level Swap Coffee client and builder.
pub mod api_client;
/// Swap Coffee API v1 requests, responses, and wire models.
pub mod v1;
