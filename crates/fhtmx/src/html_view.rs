use crate::{
    element::Element,
    html_element::*,
    node::{AsNode, HtmlNode, IntoNode},
    prelude::dc_list_row,
    render::Render,
};

pub fn html_list_row<K, V>(key: K, value: V) -> HtmlElement
where
    K: IntoNode,
    V: IntoNode,
{
    dc_list_row().add(key).add(value)
}

pub trait HtmlView {
    fn html_content(&self) -> HtmlNode;

    fn html_view(&self) -> HtmlNode {
        self.html_content()
    }

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
