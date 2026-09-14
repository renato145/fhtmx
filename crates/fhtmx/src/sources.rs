use crate::{element::Element, html_element::*};

/// Script tag with source for htmx
pub fn source_htmx() -> HtmlElement {
    script()
        .src("https://cdn.jsdelivr.net/npm/htmx.org@4.0.0")
        .set_attr(
            "integrity",
            "sha384-BvJpBiO8Kh31EqtJe5DRIeWrHWnCGkwytKs9NKFi86Hhw96dEqdEMzZDeK9iEGTc",
        )
        .set_attr("crossorigin", "anonymous")
}

/// Script tag with source for the htmx `hx-sse` extension. Enables `hx-sse:connect` and
/// `hx-sse:close` (see [`hx-sse`](https://four.htmx.org/extensions/hx-sse/)); no `hx-ext` needed.
pub fn source_htmx_sse() -> HtmlElement {
    script().src("https://cdn.jsdelivr.net/npm/htmx.org@4.0.0/dist/ext/hx-sse.min.js")
}

/// Script tag with source for the htmx `hx-ws` extension. Enables `hx-ws:connect` and `hx-ws:send`
/// (see [`hx-ws`](https://four.htmx.org/extensions/hx-ws/)); no `hx-ext` needed.
pub fn source_htmx_ws() -> HtmlElement {
    script().src("https://cdn.jsdelivr.net/npm/htmx.org@4.0.0/dist/ext/hx-ws.min.js")
}

/// Script tag with source for alpinejs
pub fn source_alpinejs() -> HtmlElement {
    script()
        .defer()
        .src("https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js")
}

/// Script tag with source for alpinejs persist extension. Make sure to include it BEFORE alpinejs
pub fn source_alpinejs_persist() -> HtmlElement {
    script()
        .defer()
        .src("https://cdn.jsdelivr.net/npm/@alpinejs/persist@3.x.x/dist/cdn.min.js")
}

/// Script tag with source for tailwindcss
pub fn source_tailwind() -> HtmlElement {
    script().src("https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4")
}

/// Link tag to import daisyui styles
pub fn daisy_link() -> HtmlElement {
    link()
        .href("https://cdn.jsdelivr.net/npm/daisyui@5")
        .rel("stylesheet")
        .typ("text/css")
}

/// Styles for typography (from <https://github.com/AnswerDotAI/typrose>)
pub fn typography_css() -> HtmlElement {
    style().add_raw(include_str!("typrose.css"))
}

/// Script to setup `sse_id` identifier
pub fn script_setup_sse() -> HtmlElement {
    script().add_raw(include_str!("setup_sse.js"))
}
