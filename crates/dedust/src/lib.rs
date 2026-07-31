#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Re-export of the shared executor and error crate used by this client.
pub use api_clients_core; // re-export
/// Top-level DeDust client and builder.
pub mod api_client;
/// DeDust asset-registry requests, responses, and wire models.
pub mod assets;
/// DeDust API v2 requests, responses, and wire models.
pub mod v2;
/// DeDust API v4 pool-registry requests, responses, and wire models.
pub mod v4;
