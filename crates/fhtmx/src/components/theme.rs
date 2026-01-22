use super::{dc_swap, icons};
use crate::{element::Element, html_element::*};

/// Script to setup theme
pub fn script_setup_theme(light_theme: &str, dark_theme: &str) -> HtmlElement {
    let js = format!(
        r#"document.addEventListener("DOMContentLoaded", function (event) {{
            if (localStorage._x_lightMode !== undefined) {{
                lightMode = localStorage._x_lightMode === "true";
                themeChange(lightMode);
            }}
        }});

        function themeChange(lightMode) {{
        document.documentElement.dataset.theme = lightMode
            ? "{light_theme}"
            : "{dark_theme}";
        }}"#
    );
    script().add_raw(js)
}

/// Requires to add `script_setup_theme()`, `source_alpinejs()` and `source_alpinejs_persist()` in
/// headers.
pub fn theme_toogle_with_size(size: u8) -> HtmlElement {
    dc_swap()
        .add_class("mx-2 swap-rotate")
        .set_attr("x-data", "{ lightMode: $persist(false) }")
        .add(
            input()
                .typ("checkbox")
                .set_attr(":checked", "lightMode")
                .set_attr(
                    "@change",
                    "lightMode = $event.target.checked; themeChange(lightMode);",
                ),
        )
        .add(icons::moon().class(format!("swap-off h-{size} w-{size} fill-current")))
        .add(icons::sun().class(format!("swap-on h-{size} w-{size} fill-current")))
}

/// Requires to add `script_setup_theme()`, `source_alpinejs()` and `source_alpinejs_persist()` in
/// headers.
#[inline]
pub fn theme_toogle() -> HtmlElement {
    theme_toogle_with_size(6)
}
