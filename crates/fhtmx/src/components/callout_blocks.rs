use super::{daisy::*, icons};
use crate::{
    children, element::Element, html_element::*, node::*, prelude::mk_dropdown, svg::SvgElement,
};
use pastey::paste;

// icon: SvgElement, content: impl IntoNode

/// Creates a new callout component
pub fn mk_callout(
    title: impl IntoNode,
    content: impl IntoNode,
    color: &str,
    collapse: bool,
) -> HtmlElement {
    let open_cls = if collapse {
        "collapse-arrow"
    } else {
        "collapse-open"
    };
    dc_collapse()
        .add_class(open_cls)
        .add_class("border border-l-6 rounded bg-base-100")
        .add_class(format!("border-{color}/50"))
        .add(input().typ("checkbox"))
        .add(
            dc_collapse_title()
                .add_class("px-2 py-1 flex items-center")
                .add_class(format!("bg-{color}/20"))
                .add(title),
        )
        .add(dc_collapse_content().add(content))
}

macro_rules! new_callout {
    ($($icon:expr; $color:literal),* $(,)?) => {
        paste! {
            $(
                #[doc = "Creates a new callout component with " $color " color."]
                pub fn [<mk_callout_ $icon>](title: Option<&str>, content: impl IntoNode, collapse: bool) -> HtmlElement {
                    let icon = icons::$icon().class("h-5 w-5").add_class(concat!("text-", $color));
                    let mut title_content = stringify!([< $icon:camel >]).to_string();
                    if let Some(s) = title {
                        title_content.push_str(": ");
                        title_content.push_str(&s);
                    }
                    let title = p()
                        .class("ml-2 pr-8 font-bold text-lg")
                        .add(title_content);
                    let title = children![icon, title];
                    let content = div().class("pt-4").add(content);
                    mk_callout(title, content, $color, collapse)
                }
            )*
        }
    };
}

new_callout!(
    note; "info",
    warning; "warning",
    important; "error",
    error; "error",
    tip; "success",
    caution; "error",
);
