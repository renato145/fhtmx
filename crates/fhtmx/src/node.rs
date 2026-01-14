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

/// Creates a `HtmlNode::Raw` node
#[inline]
pub fn raw_node(raw: impl ToString) -> HtmlNode {
    HtmlNode::Raw(raw.to_string())
}

/// Creates a `HtmlNode::Fragment` node
#[inline]
pub fn fragment(nodes: impl IntoIterator<Item = impl IntoNode>) -> HtmlNode {
    HtmlNode::Fragment(nodes.into_iter().map(|n| n.into_node()).collect())
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
