use crate::element::*;

/// Script tag with source for htmx
pub fn source_htmx() -> HtmlElement {
    script()
        .src("https://cdn.jsdelivr.net/npm/htmx.org@2.0.8/dist/htmx.min.js")
        .set_attr(
            "integrity",
            "sha384-/TgkGk7p307TH7EXJDuUlgG3Ce1UVolAOFopFekQkkXihi5u/6OCvVKyz1W+idaz",
        )
        .set_attr("crossorigin", "anonymous")
}

/// Script tag with source for the sse htmx extension
pub fn source_htmx_sse() -> HtmlElement {
    script()
        .src("https://cdn.jsdelivr.net/npm/htmx-ext-sse@2.2.4")
        .set_attr(
            "integrity",
            "sha384-A986SAtodyH8eg8x8irJnYUk7i9inVQqYigD6qZ9evobksGNIXfeFvDwLSHcp31N",
        )
        .set_attr("crossorigin", "anonymous")
}

/// Script tag with source for the ws htmx extension
pub fn source_htmx_ws() -> HtmlElement {
    script()
        .src("https://cdn.jsdelivr.net/npm/htmx-ext-ws@2.0.4")
        .set_attr(
            "integrity",
            "sha384-1RwI/nvUSrMRuNj7hX1+27J8XDdCoSLf0EjEyF69nacuWyiJYoQ/j39RT1mSnd2G",
        )
        .set_attr("crossorigin", "anonymous")
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
