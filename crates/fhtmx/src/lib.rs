pub mod attribute;
pub mod element;
pub mod html_page;
pub mod htmx;
pub mod js;
pub mod node;
pub mod render;
pub mod svg;
pub mod utils;

pub mod prelude {
    pub use crate::attribute::*;
    pub use crate::children;
    pub use crate::element::*;
    pub use crate::html_page::*;
    pub use crate::htmx::*;
    pub use crate::js::*;
    pub use crate::node;
    pub use crate::node::*;
    pub use crate::render::*;
    pub use crate::svg::*;
    pub use crate::utils::*;
}
