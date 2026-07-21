#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Re-export of the shared executor and error crate used by this client.
pub use api_clients_core;
/// Stonks requests, responses, and wire models.
pub mod api;
/// Top-level Stonks client and builder.
pub mod api_client;
