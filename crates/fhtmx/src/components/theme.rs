use super::{dc_swap, icons};
use crate::{element::Element, html_element::*};

/// Script tag with inline JS to setup the theme (see [`theme_toggle`]). The theme names are
/// passed as `data-*` attributes on the script tag; the bundled script reads them to set
/// `data-theme` on `<html>`.
pub fn script_setup_theme(light_theme: &str, dark_theme: &str) -> HtmlElement {
    script()
        .set_attr("data-light-theme", light_theme)
        .set_attr("data-dark-theme", dark_theme)
        .add_raw(include_str!("setup_theme.js"))
}

/// Requires to add [`script_setup_theme()`] in the headers. Multiple toggles in the same page are
/// kept in sync (also for htmx swapped content). The accessible name can be overridden by
/// chaining `set_attr("aria-label", ...)`.
pub fn theme_toggle_with_size(size: u8) -> HtmlElement {
    dc_swap()
        .add_class("mx-2 swap-rotate")
        .add(
            input()
                .typ("checkbox")
                .set_empty_attr("data-theme-toggle")
                .aria_label("Toggle theme"),
        )
        .add(icons::moon().class(format!("swap-off h-{size} w-{size} fill-current")))
        .add(icons::sun().class(format!("swap-on h-{size} w-{size} fill-current")))
}

/// Requires to add [`script_setup_theme()`] in the headers.
#[inline]
pub fn theme_toggle() -> HtmlElement {
    theme_toggle_with_size(6)
}
