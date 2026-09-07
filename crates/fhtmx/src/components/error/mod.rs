//! Renderable errors for htmx pages.
//!
//! `FhtmxError` responds with HTTP 200 plus `HX-Retarget`/`HX-Reswap` headers so htmx always swaps
//! the rendered error into the page. Note that htmx 4 swaps 4xx/5xx responses by default: handlers
//! that return error HTML with an error status code should design it as swap content, or opt out
//! with `hx-status:4xx="swap:none"` / the `noSwap` config.

#[cfg(feature = "actix")]
mod actix;
#[cfg(feature = "axum")]
mod axum;
mod fhtmx_error;

pub use fhtmx_error::*;
