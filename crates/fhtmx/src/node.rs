use crate::{html_element::HtmlElement, svg::SvgElement};
use uuid::Uuid;

/// A node in the HTML/SVG tree.
#[derive(Clone, Debug)]
pub enum HtmlNode {
    /// `<!DOCTYPE html>`
    Doctype,
    /// Raw HTML inserted without escaping.
    Raw(String),
    /// Escaped text content.
    Text(String),
    /// Standard HTML element.
    Element(HtmlElement),
    /// SVG element.
    SvgElement(SvgElement),
    /// Collection of nodes rendered without a wrapper.
    Fragment(Vec<HtmlNode>),
}

impl HtmlNode {
    /// Unwraps into an [`HtmlElement`] if this node is one.
    pub fn to_element(self) -> Option<HtmlElement> {
        match self {
            Self::Element(x) => Some(x),
            _ => None,
        }
    }

    /// Unwraps into an [`SvgElement`] if this node is one.
    pub fn to_svg(self) -> Option<SvgElement> {
        match self {
            Self::SvgElement(x) => Some(x),
            _ => None,
        }
    }
}

/// Creates a [`HtmlNode::Raw`] node from a string.
#[inline]
pub fn raw_node(raw: impl ToString) -> HtmlNode {
    HtmlNode::Raw(raw.to_string())
}

/// Creates a [`HtmlNode::Fragment`] from an iterator of nodes.
///
/// A fragment renders its children without a wrapper element.
///
/// # Examples
///
/// ```
/// use fhtmx::prelude::*;
///
/// let nodes = fragment(["a", "b"]);
/// assert_eq!(nodes.render(), "a\nb");
/// ```
#[inline]
pub fn fragment(nodes: impl IntoIterator<Item = impl IntoNode>) -> HtmlNode {
    HtmlNode::Fragment(nodes.into_iter().map(|n| n.into_node()).collect())
}

/// Converts a value into an [`HtmlNode`].
pub trait IntoNode {
    /// Transforms into a [`HtmlNode`].
    fn into_node(self) -> HtmlNode;
}

impl IntoNode for HtmlNode {
    fn into_node(self) -> HtmlNode {
        self
    }
}

impl<T: IntoNode> IntoNode for Vec<T> {
    fn into_node(self) -> HtmlNode {
        fragment(self)
    }
}

macro_rules! implement_into_for_display {
    ($($t:ty),* $(,)?) => {
        $(
            impl IntoNode for $t {
                fn into_node(self) -> HtmlNode {
                    HtmlNode::Text(self.to_string())
                }
            }
        )*
    };
}

implement_into_for_display!(
    String, &String, Uuid, char, &str, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128,
    usize, f32, f64
);

#[cfg(feature = "chrono_0_4")]
implement_into_for_display!(chrono::NaiveDate, chrono::DateTime<chrono::Utc>);

#[cfg(feature = "jiff_0_2")]
implement_into_for_display!(jiff::civil::Date, jiff::Timestamp);

/// Converts a value into an [`HtmlNode`] without consuming it.
pub trait AsNode {
    /// Transforms into a [`HtmlNode`] without consuming `self`.
    fn as_node(&self) -> HtmlNode;
}

impl<T: Clone + IntoNode> AsNode for T {
    fn as_node(&self) -> HtmlNode {
        self.clone().into_node()
    }
}

/// Build a list of nodes with mixed types.
///
/// # Example
///
/// ```
/// # use fhtmx::prelude::*;
/// let nodes = children!["text", p()];
/// ```
#[macro_export]
macro_rules! children {
    () => {
        Vec::<HtmlNode>::new()
    };

    ($($child:expr),* $(,)?) => {
        vec![$($child.into_node()),*]
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{element::Element, html_element::*, render::Render};

    #[test]
    fn children_macro() {
        let res = div()
            .add_children(children!["Some text", p().add("inner text"), 123456])
            .render();
        insta::assert_snapshot!(res, @r"
        <div>
          Some text
          <p>inner text</p>
          123456
        </div>
        ");
    }
}
