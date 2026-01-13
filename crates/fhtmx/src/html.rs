use paste::paste;
use std::borrow::Cow;

pub const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

pub const INLINE_ELEMENTS: &[&str] = &[
    "a", "abbr", "b", "bdo", "br", "button", "cite", "code", "em", "i", "img", "input", "kbd",
    "label", "q", "s", "samp", "select", "small", "span", "strong", "sub", "sup", "textarea",
    "time", "u", "var",
];

/// Represents a HTML element
pub struct HtmlElement {
    pub tag: &'static str,
    pub attrs: Vec<(Cow<'static, str>, String)>,
    pub children: Vec<HtmlNode>,
}

impl HtmlElement {
    pub fn new(tag: &'static str) -> Self {
        Self {
            tag,
            attrs: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn is_void_tag(&self) -> bool {
        VOID_ELEMENTS.contains(&self.tag)
    }

    pub fn is_inline_tag(&self) -> bool {
        INLINE_ELEMENTS.contains(&self.tag)
    }

    pub fn has_inline_content(&self) -> bool {
        let has_block = self.children.iter().any(|o| match o {
            HtmlNode::Element(x) => !x.is_inline_tag(),
            _ => false,
        });

        if has_block {
            false
        } else {
            // Only text and/or inline elements
            // self.children.iter().any(|c| {
            //     matches!(c, HtmlNode::Text(_))
            //         || matches!(c, HtmlNode::Element(el) if INLINE_ELEMENTS.contains(&el.tag))
            // })
            true
        }
    }

    /// Adds a child
    pub fn add_child(mut self, node: impl IntoNode) -> Self {
        self.children.push(node.into_node());
        self
    }

    /// Adds a raw html child
    pub fn add_raw(mut self, raw: impl ToString) -> Self {
        self.children.push(HtmlNode::raw(raw));
        self
    }
}

/// Types of nodes that can go inside an `Element`
pub enum HtmlNode {
    Doctype,
    Raw(String),
    Text(String),
    Element(HtmlElement),
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

macro_rules! create_tag_fn {
    ($name:ident) => {
        paste! {
            #[doc = "Creates a `" $name "` html element"]
            pub fn $name() -> HtmlElement {
                HtmlElement::new(stringify!($name))
            }
        }
    };

    ($name:ident, $($rest:ident),+) => {
        create_tag_fn!($name);
        create_tag_fn!($($rest),+);
    };
}

pub trait IntoNode {
    fn into_node(self) -> HtmlNode;
}

impl IntoNode for HtmlElement {
    fn into_node(self) -> HtmlNode {
        HtmlNode::Element(self)
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

create_tag_fn!(
    a, article, aside, audio, b, bdo, body, button, canvas, caption, cite, code, colgroup, dd,
    details, div, dl, dt, em, fieldset, figcaption, figure, footer, form, h1, h2, h3, h4, h5, h6,
    head, header, html, i, iframe, kbd, label, legend, li, mark, math, nav, noscript, object, ol,
    option, p, pre, q, s, samp, script, section, select, slot, small, span, strike, strong, style,
    sub, summary, sup, table, tbody, td, template, textarea, tfoot, th, thead, time, title, tr, u,
    ul, var, video
);

// Void elements
create_tag_fn!(
    area, base, br, col, embed, hr, img, input, link, meta, param, source, track, wbr
);

pub fn main_tag() -> HtmlElement {
    HtmlElement::new("main")
}
