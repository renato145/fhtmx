//! htmx attributes, swap strategies and request/response headers.
//!
//! Targets [htmx 4](https://four.htmx.org). Attributes that take a modifier — like the explicit
//! inheritance marker `:inherited` or `:append` — can be set generically with
//! [`Element::set_attr`](crate::element::Element::set_attr):
//!
//! ```
//! use fhtmx::prelude::*;
//!
//! let html = div().set_attr("hx-target:inherited", "#output").render();
//! assert!(html.contains(r##"hx-target:inherited="#output""##));
//! ```
//!
//! Setter values are rendered HTML-escaped. That is transparent to `htmx`: the browser decodes
//! character references in attribute values before scripts read them, so JSON/HCON values
//! round-trip exactly and user data cannot terminate the attribute or inject new ones. Use
//! [`Element::set_raw_attr`](crate::element::Element::set_raw_attr) for the rare deliberate
//! unescaped case.

use crate::{
    attribute::{AttributeValue, IntoAttributeValue},
    element::Element,
    html_element::HtmlElement,
};
use pastey::paste;

/// The `hx-swap` attribute allows you to specify how the response will be swapped in relative to the
/// target of an AJAX request. If you do not specify the option, the default is
/// `htmx.config.defaultSwap` (`innerHTML`).
#[derive(Debug, Clone, Copy)]
pub enum HXSwap {
    /// Replace the inner HTML of the target element
    InnerHtml,
    /// Replace the entire target element with the response
    OuterHTML,
    /// Morph the children of the target element, preserving as much of the existing DOM (focus,
    /// input values, CSS state) as possible
    InnerMorph,
    /// Morph the target element itself, preserving as much of the existing DOM as possible
    OuterMorph,
    /// Morph the target's attributes, then replace its children; the target stays in the DOM
    OuterSync,
    /// Replace the text content of the target element, without parsing the response as HTML
    TextContent,
    /// Insert the response before the target element
    BeforeBegin,
    /// Insert the response before the first child of the target element
    AfterBegin,
    /// Insert the response after the last child of the target element
    BeforeEnd,
    /// Insert the response after the target element
    AfterEnd,
    /// Deletes the target element regardless of the response
    Delete,
    /// Does not append content from response (out of band items will still be processed).
    None,
}

impl std::fmt::Display for HXSwap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HXSwap::InnerHtml => "innerHTML",
            HXSwap::OuterHTML => "outerHTML",
            HXSwap::InnerMorph => "innerMorph",
            HXSwap::OuterMorph => "outerMorph",
            HXSwap::OuterSync => "outerSync",
            HXSwap::TextContent => "textContent",
            HXSwap::BeforeBegin => "beforebegin",
            HXSwap::AfterBegin => "afterbegin",
            HXSwap::BeforeEnd => "beforeend",
            HXSwap::AfterEnd => "afterend",
            HXSwap::Delete => "delete",
            HXSwap::None => "none",
        };
        write!(f, "{}", s)
    }
}

impl IntoAttributeValue for HXSwap {
    fn into_attr(self) -> Option<AttributeValue> {
        Some(AttributeValue::Value(self.to_string()))
    }
}

/// The `hx-target` attribute allows you to target a different element for swapping than the one
/// issuing the AJAX request.
#[derive(Debug, Clone)]
pub enum HXTarget<'a> {
    /// Which indicates that the element that the `hx-target` attribute is on is the target.
    This,
    /// `closest <CSS selector>` which will find the closest ancestor element or itself, that matches
    /// the given CSS selector (e.g. `closest tr` will target the closest table row to the element).
    Closest(&'a str),
    /// `find <CSS selector>` which will find the first child descendant element that matches the
    /// given CSS selector.
    ///
    /// Note that htmx expects the selector along with the keyword; pass `"find .row"` directly to
    /// `hx_target` for that form, since this variant renders the bare value `find`.
    Find,
    /// `next` which resolves to `element.nextElementSibling`
    Next,
    /// `next <CSS selector>` which will scan the DOM forward for the first element that matches the
    /// given CSS selector. (e.g. `next .error` will target the closest following sibling element
    /// with error class)
    NextSelector(&'a str),
    /// `previous` which resolves to `element.previousElementSibling`
    Previous,
    /// `previous <CSS selector>` which will scan the DOM backwards for the first element that
    /// matches the given CSS selector. (e.g. `previous .error` will target the closest previous
    /// sibling with error class)
    PreviousSelector(&'a str),
}

impl std::fmt::Display for HXTarget<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HXTarget::This => "this",
            HXTarget::Closest(o) => {
                return write!(f, "closest {}", o);
            }
            HXTarget::Find => "find",
            HXTarget::Next => "next",
            HXTarget::NextSelector(o) => {
                return write!(f, "next {}", o);
            }
            HXTarget::Previous => "previous",
            HXTarget::PreviousSelector(o) => {
                return write!(f, "previous {}", o);
            }
        };
        write!(f, "{}", s)
    }
}

impl IntoAttributeValue for HXTarget<'_> {
    fn into_attr(self) -> Option<AttributeValue> {
        Some(AttributeValue::Value(self.to_string()))
    }
}

/// Request headers sent by htmx.
#[derive(Debug, Clone, Copy)]
pub enum HtmxRequestHeader {
    /// Indicates that the request is via an element using hx-boost
    HXBoosted,
    /// The current URL of the browser
    HXCurrentURL,
    /// “true” if the request is for history restoration after a miss in the history cache
    HXHistoryRestoreRequest,
    /// Always “true”
    HXRequest,
    /// `"partial"` for targeted swaps, `"full"` when targeting `body` or using `hx-select`
    HXRequestType,
    /// The source element in `tag#id` format (e.g. `button#submit`)
    HXSource,
    /// The target element in `tag#id` format (e.g. `div#results`)
    HXTarget,
}

impl std::fmt::Display for HtmxRequestHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HtmxRequestHeader::HXBoosted => "HX-Boosted",
            HtmxRequestHeader::HXCurrentURL => "HX-Current-URL",
            HtmxRequestHeader::HXHistoryRestoreRequest => "HX-History-Restore-Request",
            HtmxRequestHeader::HXRequest => "HX-Request",
            HtmxRequestHeader::HXRequestType => "HX-Request-Type",
            HtmxRequestHeader::HXSource => "HX-Source",
            HtmxRequestHeader::HXTarget => "HX-Target",
        };
        write!(f, "{}", s)
    }
}

/// Response headers understood by htmx.
#[derive(Debug, Clone, Copy)]
pub enum HtmxResponseHeader {
    /// Allows you to do a client-side redirect that does not do a full page reload
    HXLocation,
    /// Pushes a new url into the history stack
    HXPushUrl,
    /// Can be used to do a client-side redirect to a new location
    HXRedirect,
    /// If set to “true” the client-side will do a full refresh of the page
    HXRefresh,
    /// Replaces the current URL in the location bar
    HXReplaceUrl,
    /// Allows you to specify how the response will be swapped. See hx-swap for possible values
    HXReswap,
    /// A CSS selector that updates the target of the content update to a different element on the page
    HXRetarget,
    /// A CSS selector that allows you to choose which part of the response is used to be swapped in. Overrides an existing hx-select on the triggering element
    HXReselect,
    /// Allows you to trigger client-side events
    HXTrigger,
}

impl std::fmt::Display for HtmxResponseHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HtmxResponseHeader::HXLocation => "HX-Location",
            HtmxResponseHeader::HXPushUrl => "HX-Push-Url",
            HtmxResponseHeader::HXRedirect => "HX-Redirect",
            HtmxResponseHeader::HXRefresh => "HX-Refresh",
            HtmxResponseHeader::HXReplaceUrl => "HX-Replace-Url",
            HtmxResponseHeader::HXReswap => "HX-Reswap",
            HtmxResponseHeader::HXRetarget => "HX-Retarget",
            HtmxResponseHeader::HXReselect => "HX-Reselect",
            HtmxResponseHeader::HXTrigger => "HX-Trigger",
        };
        write!(f, "{}", s)
    }
}

macro_rules! set_htmx_attr {
    ($attr:ident = $name:expr; $doc:literal) => {
        paste! {
            #[doc = "Sets the `" $name "` attribute.\n\n" $doc]
            pub fn $attr(self, value: impl IntoAttributeValue) -> Self {
                self.set_attr($name, value)
            }
        }
    };

    ($attr:ident$(=$name:expr)?$(;$doc:literal)?, $($rest:ident$(=$name_rest:expr)?$(;$doc_rest:literal)?),+) => {
        set_htmx_attr!($attr$(=$name)?$(;$doc)?);
        set_htmx_attr!($($rest$(=$name_rest)?$(;$doc_rest)?),+);
    };
}

impl HtmlElement {
    set_htmx_attr!(
        hx_action = "hx-action"; "Specifies the URL of the request, typically paired with [`hx_method`](Self::hx_method); falls back to the native `action`/`method` attributes for progressive enhancement.",
        hx_boost = "hx-boost"; "Progressively enhances anchors and forms to use AJAX requests.\n\nExample: `a().hx_boost(\"true\")`",
        hx_boost_inherited = "hx-boost:inherited"; "Progressively enhances anchors and forms to use AJAX requests.\n\nExample: `a().hx_boost_inherited(\"true\")`",
        hx_config = "hx-config"; "Configures the request for the element via HCON (JSON is also accepted): `timeout`, `credentials`, `cache`, `redirect`, `referrer`, `integrity` and `validate`. The `mode` option is not allowed.\n\nExample: `button().hx_config(\"timeout:5s\")`",
        hx_confirm = "hx-confirm"; "Shows a `confirm()` dialog before issuing a request.",
        hx_delete = "hx-delete"; "Issues a `DELETE` request to the given URL. Like `hx-get`, it does not include the enclosing form's inputs; use `hx-include` if needed.",
        hx_disable = "hx-disable"; "Disables the elements matching the given CSS selector while a request is in flight. To stop htmx processing an element and its children, use [`hx_ignore`](Self::hx_ignore).",
        hx_get = "hx-get"; "Issues a `GET` request to the given URL.",
        hx_headers = "hx-headers"; "Adds name/value pairs as request headers. Takes HCON, which also accepts JSON, and supports the `js:` prefix.",
        hx_history_elt = "hx-history-elt"; "Marks the element to restore on history navigation, instead of `body`.",
        hx_ignore = "hx-ignore"; "Disables htmx processing for the element and its children (htmx 2's `hx-disable`).",
        hx_include = "hx-include"; "Includes the values of the elements matching the given CSS selector in the request.",
        hx_indicator = "hx-indicator"; "Applies `htmx-request` classes to the elements matching the given CSS selector while a request is in flight.",
        hx_method = "hx-method"; "Specifies the HTTP method of the request, paired with [`hx_action`](Self::hx_action); overrides the native `method`/`formmethod` attributes.",
        hx_morph_skip = "hx-morph-skip"; "Freezes the element (attributes and children) during `innerMorph`/`outerMorph` swaps.",
        hx_morph_skip_children = "hx-morph-skip-children"; "Updates the element's attributes but freezes its children during morph swaps.",
        hx_patch = "hx-patch"; "Issues a `PATCH` request to the given URL.",
        hx_post = "hx-post"; "Issues a `POST` request to the given URL.",
        hx_preserve = "hx-preserve"; "Preserves the element between requests.",
        hx_push_url = "hx-push-url"; "Pushes the given URL into the browser history stack after a request.",
        hx_put = "hx-put"; "Issues a `PUT` request to the given URL.",
        hx_query = "hx-query"; "Issues a `QUERY` request to the given URL, sending parameters in the request body.",
        hx_replace_url = "hx-replace-url"; "Replaces the current URL in the location bar after a request.",
        hx_select = "hx-select"; "Selects a subset of the response, matching the given CSS selector, to swap in.",
        hx_select_oob = "hx-select-oob"; "Selects out-of-band content from the response, matching the given CSS selector, to swap.",
        hx_swap = "hx-swap"; "Controls how the response content is swapped in relative to the target. See [`HXSwap`].\n\nExample: `div().hx_swap(HXSwap::OuterHTML)`",
        hx_swap_oob = "hx-swap-oob"; "Marks response content as out of band, swapping it into other parts of the page.",
        hx_sync = "hx-sync"; "Synchronizes requests between elements (e.g. to abort or replace in-flight requests).",
        hx_target = "hx-target"; "Overrides the target element of the request. See [`HXTarget`].\n\nExample: `div().hx_target(HXTarget::Closest(\"form\"))`",
        hx_trigger = "hx-trigger"; "Specifies the event(s) that trigger the request (e.g. `click`, `every 2s`).",
        hx_validate = "hx-validate"; "Controls whether inputs are validated before a request is submitted.",
        hx_vals = "hx-vals"; "Adds name/value pairs to the request parameters. Takes HCON, which also accepts JSON.\n\nExample: `div().hx_vals(r#\"{\"myVal\": \"My Value\"}\"#)`",
        hx_sse_close = "hx-sse:close"; "Closes the Server-Sent Events connection opened by [`hx_sse_connect`](Self::hx_sse_connect) when the named event arrives. Requires the `hx-sse` extension.",
        hx_sse_connect = "hx-sse:connect"; "Opens a persistent Server-Sent Events connection at the given URL. Requires the `hx-sse` extension (see [`source_htmx_sse`](crate::prelude::source_htmx_sse)). Unnamed events swap per `hx-target`/`hx-swap`; named events dispatch DOM events.",
        hx_ws_connect = "hx-ws:connect"; "Opens a WebSocket connection at the given URL. Requires the `hx-ws` extension (see [`source_htmx_ws`](crate::prelude::source_htmx_ws)). Incoming HTML swaps per `hx-target`/`hx-swap`.",
        hx_ws_send = "hx-ws:send"; "Sends the element's values and `hx-vals` as a `{headers, body}` JSON message over the nearest ancestor WebSocket connection. Requires the `hx-ws` extension."
    );

    /// Sets the `hx-status:<code>` attribute, which changes the target, swap or history handling
    /// for a response with the given status code (exact `404`, wildcard `50x` or range `5xx`).
    ///
    /// The value takes config keys: `swap:`, `target:`, `select:`, `push:`, `replace:` and
    /// `transition:`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fhtmx::prelude::*;
    ///
    /// let html = form().hx_post("/register").hx_status("422", "target:#errors").render();
    /// assert!(html.contains(r#"hx-status:422="target:#errors""#));
    /// ```
    pub fn hx_status(self, code: impl std::fmt::Display, value: impl IntoAttributeValue) -> Self {
        self.set_attr(format!("hx-status:{code}"), value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{html_element::*, render::Render};

    #[test]
    fn hx_attr_works() {
        let token = "asdoiu12309usad";
        let res = p()
            .hx_get("/some_route")
            .hx_swap(HXSwap::OuterHTML)
            .hx_headers(format!(r#"{{"Authorization": "Bearer {}"}}"#, token))
            .render();
        insta::assert_snapshot!(res, @r#"<p hx-get="/some_route" hx-swap="outerHTML" hx-headers="{&quot;Authorization&quot;: &quot;Bearer asdoiu12309usad&quot;}"></p>"#);
    }

    #[test]
    fn hx_vals_works() {
        let res = div().hx_vals(r#"{"myVal": "My Value"}"#).render();
        insta::assert_snapshot!(res, @r#"<div hx-vals="{&quot;myVal&quot;: &quot;My Value&quot;}"></div>"#);
    }

    // Item 21 (REPORT.md F07): htmx attribute setters must render HTML-escaped values. The HTML
    // parser decodes character references in attribute values before htmx reads them, so escaping
    // round-trips user data (including JSON) and makes attribute injection impossible.

    #[test]
    fn hx_vals_escapes_mixed_quotes() {
        let res = div().hx_vals(r#"{"name": "O'Reilly"}"#).render();
        insta::assert_snapshot!(res, @r#"<div hx-vals="{&quot;name&quot;: &quot;O&#x27;Reilly&quot;}"></div>"#);
    }

    #[test]
    fn hx_confirm_cannot_inject_attributes() {
        let res = div().hx_confirm("\"' data-audit='injected").render();
        insta::assert_snapshot!(res, @r#"<div hx-confirm="&quot;&#x27; data-audit=&#x27;injected"></div>"#);
        // A raw apostrophe would close the attribute early and leak the rest as new attributes.
        assert!(!res.contains('\''));
    }

    #[test]
    fn hx_vals_preserves_entity_references() {
        let res = div()
            .hx_vals(r#"{"text": "AT&T &quot;quoted&quot;"}"#)
            .render();
        insta::assert_snapshot!(res, @r#"<div hx-vals="{&quot;text&quot;: &quot;AT&amp;T &amp;quot;quoted&amp;quot;&quot;}"></div>"#);
    }

    #[test]
    fn hx_attrs_without_special_chars_are_unchanged() {
        let res = button().hx_get("/items").hx_target("#list").render();
        insta::assert_snapshot!(res, @r##"<button hx-get="/items" hx-target="#list"></button>"##);
    }
}
