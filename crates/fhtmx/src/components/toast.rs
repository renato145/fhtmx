use crate::{element::Element, html_element::*};

/// Setup alpine directives so the `el` will fade after some seconds.
/// - If oob is set to true, it will use the global toast container in the layout.
/// - If oob is false, you may want to set the `el` to be the toast container, make sure to add the
///   toast class. Also, if you want to setup the toast position you can use any of these classes:
///   toast-start, toast-center, toast-end, toast-top, toast-middle and toast-bottom
///   (https://daisyui.com/components/toast).
///
/// Make sure to add `script_setup_toast()` in your headers.
pub fn setup_toast(el: HtmlElement, oob: bool) -> HtmlElement {
    let el = el
        .set_attr("x-data", "toast")
        .set_attr("x-show", "show")
        .set_attr("x-transition:enter", "transition ease-out duration-50")
        .set_attr("x-transition:enter-start", "opacity-0")
        .set_attr("x-transition:enter-end", "opacity-100")
        .set_attr("x-transition:leave", "transition ease-in duration-1000")
        .set_attr("x-transition:leave-start", "opacity-100")
        .set_attr("x-transition:leave-end", "opacity-0");
    if oob {
        div().hx_swap_oob("afterbegin:#toast-container").add(el)
    } else {
        el
    }
}

pub trait FhtmxToast {
    /// Setup alpine directives so the `el` will fade after some seconds.
    /// - If oob is set to true, it will use the global toast container in the layout.
    /// - If oob is false, you may want to set the `el` to be the toast container, make sure to add the
    ///   toast class. Also, if you want to setup the toast position you can use any of these classes:
    ///   toast-start, toast-center, toast-end, toast-top, toast-middle and toast-bottom
    ///   (https://daisyui.com/components/toast).
    ///
    /// Make sure to add `script_setup_toast()` in your headers.
    fn setup_toast(self, oob: bool) -> HtmlElement;
}

impl FhtmxToast for HtmlElement {
    fn setup_toast(self, oob: bool) -> HtmlElement {
        setup_toast(self, oob)
    }
}
