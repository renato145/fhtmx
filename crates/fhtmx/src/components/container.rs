use crate::{element::Element, html_element::*, prelude::DaisyColor};

/// A div container `div().class("container flex flex-col")` with color settings
pub fn mk_container(color: Option<DaisyColor>) -> HtmlElement {
    div()
        .class("container flex flex-col")
        .add_opt_class(color.map(|x| x.bg_content()))
}

/// A div container `div().class("container flex flex-col mx-auto")` with color settings
pub fn mk_centered_container(color: Option<DaisyColor>) -> HtmlElement {
    mk_container(color).add_class("mx-auto")
}

/// A main container `main().class("container mx-auto")`
pub fn main_container() -> HtmlElement {
    main_tag().class("container mx-auto")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Render;

    #[test]
    fn mk_container_works() {
        let res = mk_centered_container(Some(DaisyColor::Success)).render();
        insta::assert_snapshot!(res, @r#"<div class="container flex flex-col bg-success text-success-content mx-auto"></div>"#);
    }

    #[test]
    fn main_container_works() {
        let res = main_container().render();
        insta::assert_snapshot!(res, @r#"<main class="container mx-auto"></main>"#);
    }
}
