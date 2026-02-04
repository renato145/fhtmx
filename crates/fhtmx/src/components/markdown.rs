use crate::{element::Element, html_element::*};
use pulldown_cmark::{Options, Parser};

/// Renders markdown inside a `div().class("prose")`
///
/// Other options:
/// - prose-sm, prose-lg, prose-xl, prose-2xl
/// - prose-slate, prose-zinc, prose-neutral, prose-stone, prose-invert (dark mode)
pub fn mk_markdown(md: impl AsRef<str>) -> HtmlElement {
    let md = md.as_ref().trim();
    let parser = Parser::new_ext(md, Options::all());
    let mut html_output = String::with_capacity(md.len() * 3 / 2);
    pulldown_cmark::html::push_html(&mut html_output, parser);
    div().class("prose").add_raw(html_output)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::render::Render;

    #[test]
    fn render_markdown_works() {
        let md = r#"# Some title
> Some description

- Item 1
- Item 2

**Wiii**: asd"#;
        let res = mk_markdown(md).render();
        insta::assert_snapshot!(res, @r#"
        <div class="prose"><h1>Some title</h1>
        <blockquote>
        <p>Some description</p>
        </blockquote>
        <ul>
        <li>Item 1</li>
        <li>Item 2</li>
        </ul>
        <p><strong>Wiii</strong>: asd</p>
        </div>
        "#);
    }
}
