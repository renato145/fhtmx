use crate::{element::Element, html_element::*};

/// A div container `div().class("container flex flex-col")`
pub fn mk_container() -> HtmlElement {
    div().class("container flex flex-col")
}

/// A div container `div().class("container flex flex-col mx-auto")`
pub fn mk_centered_container() -> HtmlElement {
    mk_container().add_class("mx-auto")
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
        let res = mk_centered_container().render();
        insta::assert_snapshot!(res, @r#"<div class="container flex flex-col mx-auto"></div>"#);
    }

    #[test]
    fn main_container_works() {
        let res = main_container().render();
        insta::assert_snapshot!(res, @r#"<main class="container mx-auto"></main>"#);
    }
}
