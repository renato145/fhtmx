pub mod attribute;
pub mod element;
pub mod html_page;
pub mod node;
pub mod render;
pub mod utils;

pub mod prelude {
    pub use crate::attribute::*;
    pub use crate::element::*;
    pub use crate::html_page::*;
    pub use crate::node;
    pub use crate::render::*;
    pub use crate::utils::*;
}
