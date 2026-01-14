use crate::{element::HtmlElement, svg::SvgElement};

/// Types of nodes that can go inside an `Element`
#[derive(Clone, Debug)]
pub enum HtmlNode {
    Doctype,
    Raw(String),
    Text(String),
    Element(HtmlElement),
    SvgElement(SvgElement),
    Fragment(Vec<HtmlNode>),
}

impl HtmlNode {
    #[inline]
    pub fn raw(raw: impl ToString) -> Self {
        Self::Raw(raw.to_string())
    }

    #[inline]
    pub fn text(raw: impl ToString) -> Self {
        Self::Text(raw.to_string())
    }
}

pub trait IntoNode {
    /// Transforms into a `HtmlNode`
    fn into_node(self) -> HtmlNode;
}

impl IntoNode for HtmlElement {
    fn into_node(self) -> HtmlNode {
        HtmlNode::Element(self)
    }
}

impl IntoNode for SvgElement {
    fn into_node(self) -> HtmlNode {
        HtmlNode::SvgElement(self)
    }
}

impl IntoNode for HtmlNode {
    fn into_node(self) -> HtmlNode {
        self
    }
}

impl<T: ToString> IntoNode for T {
    fn into_node(self) -> HtmlNode {
        HtmlNode::Text(self.to_string())
    }
}
