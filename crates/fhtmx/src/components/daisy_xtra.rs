use super::daisy::*;
use crate::{element::Element, html_element::*, node::*, utils::random_id};
use std::borrow::Cow;

/// Creates a new Daisy dropdown component.
/// Dropdown can open a menu or any other element when the button is clicked
pub fn mk_dropdown<T, I, Q>(
    summary_txt: &str,
    items: I,
    summary_cls: T,
    content_cls: Q,
) -> HtmlElement
where
    T: Into<Cow<'static, str>>,
    I: IntoIterator,
    I::Item: IntoNode,
    Q: Into<Cow<'static, str>>,
{
    dc_dropdown()
        .add(summary().class(summary_cls).add(summary_txt))
        .add(
            dc_dropdown_content()
                .add_class("menu")
                .add_class(content_cls)
                .add_children(items),
        )
}

/// Creates a new Daisy fab component.
/// FAB (Floating Action Button) stays in the bottom corner of screen
pub fn mk_fab<T, I>(btn_txt: &str, items: I, btn_cls: T) -> HtmlElement
where
    T: Into<Cow<'static, str>>,
    I: IntoIterator,
    I::Item: IntoNode,
{
    dc_fab()
        .add(
            div()
                .tabindex(0)
                .role("button")
                .class("btn")
                .add_class(btn_cls)
                .add(btn_txt),
        )
        .add_children(items)
}

/// Creates a new Daisy swap component.
/// Swap allows you to toggle the visibility of two elements
pub fn mk_swap<T, Q>(on_txt: T, off_txt: Q) -> HtmlElement
where
    T: IntoNode,
    Q: IntoNode,
{
    dc_swap()
        .add(input().typ("checkbox"))
        .add(dc_swap_on().add(on_txt))
        .add(dc_swap_off().add(off_txt))
}

pub fn mk_accordion<I, T, C>(
    title_contents: I,
    cls: &str,
    title_cls: &str,
    content_cls: &str,
    name: Option<String>,
) -> HtmlElement
where
    I: IntoIterator<Item = (T, C)>,
    T: IntoNode,
    C: IntoNode,
{
    let name = name.unwrap_or_else(|| random_id("acc"));
    div().add_children(
        title_contents
            .into_iter()
            .enumerate()
            .map(|(i, (title, content))| {
                dc_collapse()
                    .add_class(cls.to_string())
                    .add(input().typ("radio").name(&name).set_attr("checked", i == 0))
                    .add(
                        dc_collapse_title()
                            .add_class(title_cls.to_string())
                            .add(title),
                    )
                    .add(
                        dc_collapse_content()
                            .add_class(content_cls.to_string())
                            .add(content),
                    )
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Render;

    #[test]
    fn mk_dropdown_works() {
        let res = mk_dropdown(
            "open or close",
            ["Item 1", "Item 2"].map(|o| li().add(a().add(o))),
            "btn m-1",
            "bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm",
        )
        .render();
        insta::assert_snapshot!(res, @r#"
        <details class="dropdown">
          <summary class="btn m-1">open or close</summary>
          <ul class="dropdown-content menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm">
            <li><a>Item 1</a></li>
            <li><a>Item 2</a></li>
          </ul>
        </details>
        "#);
    }

    #[test]
    fn mk_fab_works() {
        let res = mk_fab(
            "F",
            ["A", "B", "C"].map(|o| dc_btn().add_class("btn-lg btn-circle").add(o)),
            "btn-lg btn-circle btn-primary",
        )
        .render();
        insta::assert_snapshot!(res, @r#"
        <div class="fab">
          <div class="btn btn-lg btn-circle btn-primary" tabindex="0" role="button">F</div>
          <button class="btn btn-lg btn-circle">A</button>
          <button class="btn btn-lg btn-circle">B</button>
          <button class="btn btn-lg btn-circle">C</button>
        </div>
        "#);
    }

    #[test]
    fn mk_swap_works() {
        let res = mk_swap("ON", "OFF").render();
        insta::assert_snapshot!(res, @r#"
        <label class="swap">
          <input type="checkbox" />
          <div class="swap-on">ON</div>
          <div class="swap-off">OFF</div>
        </label>
        "#);
    }

    #[test]
    fn mk_accordion_works() {
        let res = mk_accordion(
            [
                ("Title 1", "Some content 1"),
                ("Title 2", "Some content 2"),
                ("Title 3", "Some content 3"),
            ],
            "bg-base-100 border border-base-300",
            "font-semibold",
            "text-sm",
            Some("my-accordion".to_string()),
        )
        .render();
        insta::assert_snapshot!(res, @r#"
        <div>
          <div class="collapse bg-base-100 border border-base-300">
            <input type="radio" name="my-accordion" checked />
            <div class="collapse-title font-semibold">Title 1</div>
            <div class="collapse-content text-sm">Some content 1</div>
          </div>
          <div class="collapse bg-base-100 border border-base-300">
            <input type="radio" name="my-accordion" />
            <div class="collapse-title font-semibold">Title 2</div>
            <div class="collapse-content text-sm">Some content 2</div>
          </div>
          <div class="collapse bg-base-100 border border-base-300">
            <input type="radio" name="my-accordion" />
            <div class="collapse-title font-semibold">Title 3</div>
            <div class="collapse-content text-sm">Some content 3</div>
          </div>
        </div>
        "#);
    }
}
