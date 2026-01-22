use super::{daisy::*, icons};
use crate::{element::Element, html_element::*, node::*, svg::SvgElement};
use pastey::paste;

/// Creates a new Daisy alert component.
/// Alert informs users about important events
pub fn mk_alert(icon: SvgElement, content: impl IntoNode) -> HtmlElement {
    dc_alert()
        .role("alert")
        .add(icon.class("h-6 w-6"))
        .add(content)
}

macro_rules! new_alert {
    ($($color:expr),* $(,)?) => {
        paste! {
            $(
                #[doc = "Creates a new Daisy alert component with " $color " color."]
                pub fn [<mk_alert_ $color>](s: impl ToString) -> HtmlElement {
                    let child = span().class("whitespace-pre-wrap").add(s.to_string());
                    mk_alert(icons::$color(), child).add_class(concat!("alert-", stringify!($color)))
                }
            )*
        }
    };
}

new_alert!(info, success, warning, error);

#[cfg(test)]
mod test {
    use super::*;
    use crate::render::Render;

    #[test]
    fn alert_works() {
        let res = mk_alert_error("Some error").render();
        insta::assert_snapshot!(res, @r#"
        <div class="alert" role="alert">
          <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
          </svg>
          <span class="whitespace-pre-wrap">Some error</span>
        </div>
        "#);
    }
}
