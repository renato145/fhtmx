//! Axum integration for fhtmx.
//!
//! Provides response helpers, SSE utilities, and the `HxRequest` extractor.

/// Response helpers for Axum.
pub mod response;
/// Server-sent events for Axum.
pub mod sse;
/// Utility types for Axum integration.
pub mod utils;
// TODO: ws

/// Re-exports the Axum integration types.
pub mod prelude {
    pub use crate::response::*;
    // pub use crate::sse::*;
    pub use crate::utils::*;
}
