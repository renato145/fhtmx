//! Actix-web integration for fhtmx.
//!
//! Provides response helpers, SSE utilities, and the `HXRequest` header extractor.

/// Response helpers for Actix.
pub mod response;
/// Server-sent events for Actix.
pub mod sse;
/// Utility types for Actix integration.
pub mod utils;
// TODO: ws

/// Re-exports the Actix integration types.
pub mod prelude {
    pub use crate::response::*;
    pub use crate::sse::*;
    pub use crate::utils::*;
}
