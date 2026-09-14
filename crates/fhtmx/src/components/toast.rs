use crate::{element::Element, html_element::*};

/// Script to setup toast
pub fn script_setup_toast() -> HtmlElement {
    script().add_raw(include_str!("setup_toast.js"))
}

/// Marks the `el` so the bundled script will fade it after some seconds and then remove it.
/// - If oob is set to true, it will use the global toast container in the layout.
/// - If oob is false, you may want to set the `el` to be the toast container, make sure to add the
///   toast class. Also, if you want to setup the toast position you can use any of these classes:
///   toast-start, toast-center, toast-end, toast-top, toast-middle and toast-bottom
///   (<https://daisyui.com/components/toast>).
///
/// Hovering the toast or focusing its content pauses its dismissal.
///
/// Make sure to add `script_setup_toast()` in your headers.
///
/// # Examples
///
/// ```
/// use fhtmx::prelude::*;
///
/// let html = mk_alert_error("Some error.").setup_toast(false).render();
/// assert!(html.contains("data-toast"));
/// ```
pub fn setup_toast(el: HtmlElement, oob: bool) -> HtmlElement {
    let el = el.set_empty_attr("data-toast");
    if oob {
        div().hx_swap_oob("afterbegin:#toast-container").add(el)
    } else {
        el
    }
}

/// Extension trait for setting up toast behavior on [`HtmlElement`].
pub trait FhtmxToast {
    /// Marks the `el` so the bundled script will fade it after some seconds and then remove it.
    /// - If oob is set to true, it will use the global toast container in the layout.
    /// - If oob is false, you may want to set the `el` to be the toast container, make sure to add the
    ///   toast class. Also, if you want to setup the toast position you can use any of these classes:
    ///   toast-start, toast-center, toast-end, toast-top, toast-middle and toast-bottom
    ///   (<https://daisyui.com/components/toast>).
    ///
    /// Make sure to add `script_setup_toast()` in your headers.
    fn setup_toast(self, oob: bool) -> HtmlElement;
}

impl FhtmxToast for HtmlElement {
    fn setup_toast(self, oob: bool) -> HtmlElement {
        setup_toast(self, oob)
    }
}
