mod attribute;
mod components;
mod element;
mod html_element;
mod html_page;
mod htmx;
mod js;
mod node;
mod render;
mod sources;
mod svg;
mod utils;

pub mod prelude {
    pub use crate::attribute::*;
    pub use crate::children;
    pub use crate::components::*;
    pub use crate::element::*;
    pub use crate::html_element::*;
    pub use crate::html_page::*;
    pub use crate::htmx::*;
    pub use crate::js::*;
    pub use crate::node::*;
    pub use crate::render::*;
    pub use crate::sources::*;
    pub use crate::svg::*;
    pub use crate::utils::*;
}
