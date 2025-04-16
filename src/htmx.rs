use crate::elements::HtmlElement;
use paste::paste;

/// The hx-swap attribute allows you to specify how the response will be swapped in relative to the
/// target of an AJAX request. If you do not specify the option, the default is
/// htmx.config.defaultSwapStyle (innerHTML).
#[derive(Debug, Clone, Copy)]
pub enum HXSwap {
    /// Replace the inner html of the target element
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
            HXSwap::InnerHtml => "innerHtml",
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

/// The hx-target attribute allows you to target a different element for swapping than the one
/// issuing the AJAX request.
#[derive(Debug, Clone)]
pub enum HXTarget<'a> {
    /// Which indicates that the element that the hx-target attribute is on is the target.
    This,
    /// closest <CSS selector> which will find the closest ancestor element or itself, that matches
    /// the given CSS selector (e.g. closest tr will target the closest table row to the element).
    Closest(&'a str),
    /// find <CSS selector> which will find the first child descendant element that matches the
    /// given CSS selector.
    Find,
    /// next which resolves to element.nextElementSibling
    Next,
    /// next <CSS selector> which will scan the DOM forward for the first element that matches the
    /// given CSS selector. (e.g. next .error will target the closest following sibling element
    /// with error class)
    NextSelector(&'a str),
    /// previous which resolves to element.previousElementSibling
    Previous,
    /// previous <CSS selector> which will scan the DOM backwards for the first element that
    /// matches the given CSS selector. (e.g. previous .error will target the closest previous
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

/// The hx-trigger attribute allows you to specify what triggers an AJAX request. A trigger value
/// can be one of the following:
///
/// - An event name (e.g. “click” or “my-custom-event”) followed by an event filter and a set of
///   event modifiers
/// - A polling definition of the form every <timing declaration>
/// - A comma-separated list of such events
#[derive(Debug, Clone)]
pub enum HXTrigger {
    // TODO:implement
}

macro_rules! set_htmx_attr {
    ($attr:ident = $name:expr) => {
        paste! {
            #[doc = "Sets the `" $name "` attribute."]
            pub fn $attr(self, value: impl ToString) -> Self {
                self.set_attr($name, value)
            }
        }
    };

    ($attr:ident = $name:expr, $($rest:ident = $rest_names:expr),+) => {
        set_htmx_attr!($attr = $name);
        set_htmx_attr!($($rest = $rest_names),+);
    };
}

// TODO: add documentation for each attr
impl<T, G> HtmlElement<T, G> {
    set_htmx_attr!(
        hx_confirm = "hx-confirm",
        hx_delete = "hx-delete",
        hx_disable = "hx-disable",
        hx_disabled_elt = "hx-disabled-elt",
        hx_ext = "hx-ext",
        hx_get = "hx-get",
        hx_headers = "hx-headers",
        hx_history = "hx-history",
        hx_history_elt = "hx-history-elt",
        hx_include = "hx-include",
        hx_indicator = "hx-indicator",
        hx_inherit = "hx-inherit",
        hx_params = "hx-params",
        hx_patch = "hx-patch",
        hx_post = "hx-post",
        hx_preserve = "hx-preserve",
        hx_prompt = "hx-prompt",
        hx_push_url = "hx-push-url",
        hx_put = "hx-put",
        hx_replace_url = "hx-replace-url",
        hx_request = "hx-request",
        hx_select = "hx-select",
        hx_select_oob = "hx-select-oob",
        hx_swap = "hx-swap",
        hx_swap_oob = "hx-swap-oob",
        hx_sync = "hx-sync",
        hx_target = "hx-target",
        hx_trigger = "hx-trigger",
        hx_validate = "hx-validate",
        hx_vals = "hx-vals"
    );
}

#[cfg(test)]
mod test {
    use crate::prelude::{HtmlRender, p};

    #[test]
    fn hx_attr_works() {
        let res = p().hx_get("/some_route").render();
        println!("{}", res);
    }
}
