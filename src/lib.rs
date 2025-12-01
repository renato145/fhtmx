#![allow(clippy::uninlined_format_args)]

pub mod elements;
pub mod html_page;
pub mod htmx;
pub mod js;
pub mod svg;

pub mod prelude {
    pub use crate::elements::*;
    pub use crate::html_page::*;
    pub use crate::htmx::*;
    pub use crate::js::*;
    pub use crate::svg::*;
}
