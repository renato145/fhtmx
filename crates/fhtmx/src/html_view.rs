use crate::{
    element::Element,
    html_element::*,
    node::{AsNode, HtmlNode, IntoNode},
    prelude::dc_list_row,
    render::Render,
};

/// Creates a key/value list row for use in [`HtmlView`] renders.
pub fn html_list_row<K, V>(key: K, value: V) -> HtmlElement
where
    K: IntoNode,
    V: IntoNode,
{
    dc_list_row().add(key).add(value)
}

/// Renders a type as HTML (table, list, or card).
///
/// The [`HtmlView`](derive@crate::prelude::HtmlView) derive implements this trait for named structs,
/// generating a list, table or right-aligned table layout (see the `mode` attribute).
///
/// # Examples
///
/// ```
/// use fhtmx::prelude::*;
///
/// #[derive(HtmlView)]
/// #[html_view(title = "User", mode = "table")]
/// struct User {
///     name: String,
///     #[html_view(alias = "Age")]
///     age: u8,
/// }
///
/// let html = User { name: "Ana".to_string(), age: 30 }.render_view();
/// assert!(html.contains("<td>Ana</td>"));
/// ```
pub trait HtmlView {
    /// Returns the inner content without wrapping.
    fn html_content(&self) -> HtmlNode;

    /// Returns the full view (wrapped in a card by default for derived impls).
    fn html_view(&self) -> HtmlNode {
        self.html_content()
    }

    /// Renders the view to a string.
    fn render_view(&self) -> String {
        self.html_view().render()
    }
}

impl<T: AsNode> HtmlView for T {
    fn html_content(&self) -> HtmlNode {
        self.as_node()
    }
}

impl<T: HtmlView> HtmlView for Option<T> {
    fn html_content(&self) -> HtmlNode {
        match self {
            Some(x) => x.html_view(),
            None => "-".into_node(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn html_view_works_for_opt() {
        let x = Some("wiii");
        let _ = x.html_view();
        let x = Some("wiii".to_string());
        let _ = x.html_view();
        let _ = x.as_ref().html_view();
    }
}
