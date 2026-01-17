use crate::{element::Element, html_element::*, prelude::DaisyColor};

pub fn mk_container(color: Option<DaisyColor>) -> HtmlElement {
    div()
        .class("container flex flex-col")
        .add_opt_class(color.map(|x| x.bg_content()))
}

pub fn mk_centered_container(color: Option<DaisyColor>) -> HtmlElement {
    mk_container(color).add_class("mx-auto")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Render;

    #[test]
    fn mk_container_works() {
        let res = mk_centered_container(Some(DaisyColor::Primary)).render();
        insta::assert_snapshot!(res, @r#"<div class="container flex flex-col bg-primary text-primary-content mx-auto"></div>"#);
    }
}
