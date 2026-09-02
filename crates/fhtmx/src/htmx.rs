use crate::{
    attribute::{AttributeValue, IntoAttributeValue},
    element::Element,
    html_element::HtmlElement,
};
use pastey::paste;

/// The `hx-swap` attribute allows you to specify how the response will be swapped in relative to the
/// target of an AJAX request. If you do not specify the option, the default is
/// `htmx.config.defaultSwapStyle` (`innerHTML`).
#[derive(Debug, Clone, Copy)]
pub enum HXSwap {
    /// Replace the inner HTML of the target element
    InnerHtml,
    /// Replace the entire target element with the response
    OuterHTML,
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
    /// “true” if the request is for history restoration after a miss in the local history cache
    HXHistoryRestoreRequest,
    /// The user response to an hx-prompt
    HXPrompt,
    /// Always “true”
    HXRequest,
    /// The id of the target element if it exists
    HXTarget,
    /// The name of the triggered element if it exists
    HXTriggerName,
    /// The id of the triggered element if it exists
    HXTrigger,
}

impl std::fmt::Display for HtmxRequestHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HtmxRequestHeader::HXBoosted => "HX-Boosted",
            HtmxRequestHeader::HXCurrentURL => "HX-Current-URL",
            HtmxRequestHeader::HXHistoryRestoreRequest => "HX-History-Restore-Request",
            HtmxRequestHeader::HXPrompt => "HX-Prompt",
            HtmxRequestHeader::HXRequest => "HX-Request",
            HtmxRequestHeader::HXTarget => "HX-Target",
            HtmxRequestHeader::HXTriggerName => "HX-Trigger-Name",
            HtmxRequestHeader::HXTrigger => "HX-Trigger",
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
    /// Allows you to trigger client-side events after the settle step
    HXTriggerAfterSettle,
    /// Allows you to trigger client-side events after the swap step
    HXTriggerAfterSwap,
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
            HtmxResponseHeader::HXTriggerAfterSettle => "HX-Trigger-After-Settle",
            HtmxResponseHeader::HXTriggerAfterSwap => "HX-Trigger-After-Swap",
        };
        write!(f, "{}", s)
    }
}

// TODO: simplify
macro_rules! set_htmx_attr {
    ($attr:ident = $name:expr; $doc:literal) => {
        paste! {
            #[doc = "Sets the `" $name "` attribute.\n\n" $doc]
            pub fn $attr(self, value: impl IntoAttributeValue) -> Self {
                self.set_raw_attr($name, value)
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
        hx_boost = "hx-boost"; "Progressively enhances anchors and forms to use AJAX requests.\n\nExample: `a().hx_boost(\"true\")`",
        hx_confirm = "hx-confirm"; "Shows a `confirm()` dialog before issuing a request.",
        hx_delete = "hx-delete"; "Issues a `DELETE` request to the given URL.",
        hx_disable = "hx-disable"; "Disables htmx processing for the element and its children.",
        hx_disabled_elt = "hx-disabled-elt"; "Disables the elements matching the given CSS selector while a request is in flight.",
        hx_ext = "hx-ext"; "Enables htmx extensions for the element and its children.",
        hx_get = "hx-get"; "Issues a `GET` request to the given URL.",
        hx_headers = "hx-headers"; "Adds JSON name/value pairs as request headers.",
        hx_history = "hx-history"; "Controls whether the element's state is saved in the local history cache.",
        hx_history_elt = "hx-history-elt"; "Marks the element to snapshot for history restoration.",
        hx_include = "hx-include"; "Includes the values of the elements matching the given CSS selector in the request.",
        hx_indicator = "hx-indicator"; "Applies `htmx-request` classes to the elements matching the given CSS selector while a request is in flight.",
        hx_inherit = "hx-inherit"; "Controls inheritance of htmx attributes from ancestor elements.",
        hx_params = "hx-params"; "Filters the parameters that are submitted with the request.",
        hx_patch = "hx-patch"; "Issues a `PATCH` request to the given URL.",
        hx_post = "hx-post"; "Issues a `POST` request to the given URL.",
        hx_preserve = "hx-preserve"; "Preserves the element between requests.",
        hx_prompt = "hx-prompt"; "Shows a `prompt()` dialog before issuing a request; the result is sent as the `HX-Prompt` header.",
        hx_push_url = "hx-push-url"; "Pushes the given URL into the browser history stack after a request.",
        hx_put = "hx-put"; "Issues a `PUT` request to the given URL.",
        hx_replace_url = "hx-replace-url"; "Replaces the current URL in the location bar after a request.",
        hx_request = "hx-request"; "Configures request aspects via a JSON value: `timeout`, `credentials` and `noHeaders`.",
        hx_select = "hx-select"; "Selects a subset of the response, matching the given CSS selector, to swap in.",
        hx_select_oob = "hx-select-oob"; "Selects out-of-band content from the response, matching the given CSS selector, to swap.",
        hx_swap = "hx-swap"; "Controls how the response content is swapped in relative to the target. See [`HXSwap`].\n\nExample: `div().hx_swap(HXSwap::OuterHTML)`",
        hx_swap_oob = "hx-swap-oob"; "Marks response content as out of band, swapping it into other parts of the page.",
        hx_sync = "hx-sync"; "Synchronizes requests between elements (e.g. to abort or replace in-flight requests).",
        hx_target = "hx-target"; "Overrides the target element of the request. See [`HXTarget`].\n\nExample: `div().hx_target(HXTarget::Closest(\"form\"))`",
        hx_trigger = "hx-trigger"; "Specifies the event(s) that trigger the request (e.g. `click`, `every 2s`).",
        hx_validate = "hx-validate"; "Controls whether inputs are validated before a request is submitted.",
        hx_vals = "hx-vals"; "Adds JSON name/value pairs as request parameters.\n\nExample: `div().hx_vals(r#\"{\"myVal\": \"My Value\"}\"#)`",
        sse_connect = "sse-connect"; "Connects to a Server-Sent Events source at the given URL.",
        sse_swap = "sse-swap"; "Swaps in content from the Server-Sent Events event with the given name.",
        ws_connect = "ws-connect"; "Connects to a WebSocket endpoint at the given URL.",
        ws_send = "ws-send"; "Sends the element's input values over the WebSocket connection on trigger."
    );
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
        insta::assert_snapshot!(res, @r#"<p hx-get="/some_route" hx-swap="outerHTML" hx-headers='{"Authorization": "Bearer asdoiu12309usad"}'></p>"#);
    }

    #[test]
    fn hx_vals_works() {
        let res = div().hx_vals(r#"{"myVal": "My Value"}"#).render();
        insta::assert_snapshot!(res, @r#"<div hx-vals='{"myVal": "My Value"}'></div>"#);
    }
}
