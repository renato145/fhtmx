use crate::{html_element::HtmlElement, svg::SvgElement};

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
    pub fn to_element(self) -> Option<HtmlElement> {
        match self {
            Self::Element(x) => Some(x),
            _ => None,
        }
    }

    pub fn to_svg(self) -> Option<SvgElement> {
        match self {
            Self::SvgElement(x) => Some(x),
            _ => None,
        }
    }
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

macro_rules! implement_for_display {
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

implement_for_display!(
    char, &str, &String, String, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
    f32, f64
);

/// Build a list of nodes with mixed types.
///
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
