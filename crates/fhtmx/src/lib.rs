//! Create HTML with Rust, with support for htmx attributes and DaisyUI components.
//!
//! # Quick start
//!
//! ```
//! use fhtmx::prelude::*;
//!
//! let html = div()
//!     .class("container")
//!     .add(h1().add("Hello, world!"))
//!     .add(p().add("Some text"))
//!     .render();
//! ```
//!
//! # Features
//!
//! - `anyhow` (default): Enables `anyhow::Error` conversion support.
//! - `chrono_0_4`: Enables `IntoNode` for `chrono` types.
//! - `jiff_0_2`: Enables `IntoNode` for `jiff` types.
//! - `actix`: Enables Actix-web response integration.
//! - `axum`: Enables Axum response integration.

mod attribute;
mod components;
mod element;
mod html_element;
mod html_page;
mod html_view;
mod htmx;
mod js;
mod node;
mod render;
mod sources;
mod svg;
mod url_query;
mod utils;

/// Re-exports the most commonly used types and traits.
pub mod prelude {
    pub use crate::attribute::*;
    pub use crate::children;
    pub use crate::components::*;
    pub use crate::element::*;
    pub use crate::html_element::*;
    pub use crate::html_page::*;
    pub use crate::html_view::*;
    pub use crate::htmx::*;
    pub use crate::js::*;
    pub use crate::node::*;
    pub use crate::render::*;
    pub use crate::sources::*;
    pub use crate::svg::*;
    pub use crate::url_query::*;
    pub use crate::utils::*;
    pub use fhtmx_derive::HtmlView;
}
