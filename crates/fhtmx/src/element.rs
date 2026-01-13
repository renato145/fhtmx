use crate::{
    attribute::{AttributeValue, IntoAttributeValue},
    node::{HtmlNode, IntoNode},
};
use indexmap::IndexMap;
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
#[derive(Clone, Debug)]
pub struct HtmlElement {
    pub tag: &'static str,
    pub attrs: IndexMap<Cow<'static, str>, AttributeValue>,
    pub children: Vec<HtmlNode>,
}

// TODO: implement commented methods
impl HtmlElement {
    pub fn new(tag: &'static str) -> Self {
        Self {
            tag,
            attrs: IndexMap::new(),
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

    /// Adds a raw html child
    pub fn add_raw(mut self, raw: impl ToString) -> Self {
        self.children.push(HtmlNode::raw(raw));
        self
    }

    /// Adds a child
    pub fn add_child(mut self, node: impl IntoNode) -> Self {
        let node = node.into_node();
        match node {
            HtmlNode::Fragment(mut x) => {
                self.children.append(&mut x);
            }
            x => self.children.push(x),
        }
        self
    }

    // pub fn add_child_if<C: HtmlRender + 'static>(self, cond: bool, child: C) -> Self {
    //     if cond {
    //         return self.add_child(child);
    //     }
    //     self
    // }
    //
    // /// Adds child if it contains a value
    // pub fn add_opt_child<C: HtmlRender + 'static>(self, child: Option<C>) -> Self {
    //     if let Some(child) = child {
    //         return self.add_child(child);
    //     }
    //     self
    // }
    //
    // /// Adds child if the closure returns a value
    // pub fn maybe_add_child<F, C>(self, child: F) -> Self
    // where
    //     F: FnOnce() -> Option<C>,
    //     C: HtmlRender + 'static,
    // {
    //     self.add_opt_child(child())
    // }

    /// Add children
    pub fn add_children(mut self, nodes: impl IntoIterator<Item = impl IntoNode>) -> Self {
        self.children
            .extend(nodes.into_iter().map(|n| n.into_node()));
        self
    }

    // pub fn add_children_if(self, cond: bool, children: HtmlElements) -> Self {
    //     if cond {
    //         return self.add_children(children);
    //     }
    //     self
    // }
    //
    // /// Adds children if it contains a value
    // pub fn add_opt_children(self, children: Option<HtmlElements>) -> Self {
    //     if let Some(children) = children {
    //         return self.add_children(children);
    //     }
    //     self
    // }
    //
    // /// Adds children if the closure returns a value
    // pub fn maybe_add_children<F>(self, children: F) -> Self
    // where
    //     F: FnOnce() -> Option<HtmlElements>,
    // {
    //     self.add_opt_children(children())
    // }

    pub fn set_attr<K, V>(mut self, attr: K, value: V) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        if let Some(v) = value.into_attr() {
            self.attrs.insert(attr.into(), v);
        }
        self
    }

    // pub fn set_attr_if(self, cond: bool, attr: impl ToString, value: impl ToString) -> Self {
    //     if cond {
    //         return self.set_attr(attr, value);
    //     }
    //     self
    // }
    //
    // /// Sets an attribute if contains a value
    // pub fn set_opt_attr(self, attr: impl ToString, value: Option<impl ToString>) -> Self {
    //     if let Some(value) = value {
    //         return self.set_attr(attr, value);
    //     }
    //     self
    // }
    //
    // /// Sets an attribute if the closure returns a value
    // pub fn maybe_set_attr<F, R>(self, attr: impl ToString, value: F) -> Self
    // where
    //     F: FnOnce() -> Option<R>,
    //     R: ToString,
    // {
    //     self.set_opt_attr(attr, value())
    // }

    pub fn set_empty_attr(mut self, attr: impl Into<Cow<'static, str>>) -> Self {
        self.attrs.insert(attr.into(), AttributeValue::Empty);
        self
    }

    // pub fn set_empty_attr_if(self, cond: bool, attr: impl ToString) -> Self {
    //     if cond {
    //         return self.set_empty_attr(attr);
    //     }
    //     self
    // }
    //
    // /// Sets an empty_attrs if contains an attr
    // pub fn set_opt_empty_attr(self, attr: Option<impl ToString>) -> Self {
    //     if let Some(attr) = attr {
    //         return self.set_empty_attr(attr);
    //     }
    //     self
    // }
    //
    // /// Sets an empty attribute if the closure returns an attr
    // pub fn maybe_set_empty_attr<F, R>(self, attr: F) -> Self
    // where
    //     F: FnOnce() -> Option<R>,
    //     R: ToString,
    // {
    //     self.set_opt_empty_attr(attr())
    // }
    //
    // pub fn have_class(&self, class: impl AsRef<str>) -> bool {
    //     let class = class.as_ref();
    //     self.attrs
    //         .get("class")
    //         .map(|x| x.split_whitespace().any(|o| o == class))
    //         .unwrap_or_default()
    // }
    //
    // pub fn add_class<C: AsRef<str> + ToString>(mut self, class: C) -> Self {
    //     if !self.have_class(&class) {
    //         self.attrs
    //             .entry("class".to_string())
    //             .and_modify(|x| x.push_str(&format!(" {}", class.as_ref())))
    //             .or_insert_with(|| class.to_string());
    //     }
    //     self
    // }
    //
    // pub fn remove_class(mut self, class: impl AsRef<str>) -> Self {
    //     if let Some(x) = self.attrs.get_mut("class") {
    //         *x = x
    //             .split_whitespace()
    //             .filter(|&o| o != class.as_ref())
    //             .collect::<Vec<_>>()
    //             .join(" ");
    //     }
    //     self
    // }
    //
    // pub fn toogle_class<C: AsRef<str> + ToString>(self, class: C) -> Self {
    //     if self.have_class(&class) {
    //         self.remove_class(class)
    //     } else {
    //         self.add_class(class)
    //     }
    // }
    //
    // pub fn have_attrs(&self) -> bool {
    //     !(self.attrs.is_empty() && self.empty_attrs.is_empty())
    // }
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

/// Creates a `main` html element
pub fn main_tag() -> HtmlElement {
    HtmlElement::new("main")
}

macro_rules! set_attr {
    ($attr:ident = $name:expr; eg = $eg:expr) => {
        paste! {
            #[doc = "Sets the `" $name "` attribute.\nExample: `" $eg "`"]
            pub fn $attr(self, value: impl IntoAttributeValue) -> Self {
                self.set_attr($name, value)
            }
        }
    };

    ($attr:ident = $name:expr) => {
        paste! {
            #[doc = "Sets the `" $name "` attribute."]
            pub fn $attr(self, value: impl IntoAttributeValue) -> Self {
                self.set_attr($name, value)
            }
        }
    };

    ($attr:ident) => {
        paste! {
            #[doc = "Sets the `" $attr "` attribute."]
            pub fn $attr(self, value: impl IntoAttributeValue) -> Self {
                self.set_attr(stringify!([< $attr:lower >]), value)
            }
        }
    };

    ($attr:ident$(=$name:expr)?$(;eg=$eg:expr)?, $($rest:ident$(=$name_rest:expr)?$(;eg=$eg_rest:expr)?),+) => {
        set_attr!($attr$(=$name)?$(;eg=$eg)?);
        set_attr!($($rest$(=$name_rest)?$(;eg=$eg_rest)?),+);
    };
}

// pub(crate) use set_attr;

macro_rules! set_empty_attr {
    ($attr:ident = $name:expr) => {
        paste! {
            #[doc = "Sets the `" $name "` empty attribute."]
            pub fn $attr(self) -> Self {
                self.set_empty_attr($name)
            }
        }
    };

    ($attr:ident) => {
        paste! {
            #[doc = "Sets the `" $attr "` empty attribute."]
            pub fn $attr(self) -> Self {
                self.set_empty_attr(stringify!([< $attr:lower >]))
            }
        }
    };

    ($attr:ident$(=$name:expr)?, $($rest:ident$(=$name_rest:expr)?),+) => {
        set_empty_attr!($attr$(=$name)?);
        set_empty_attr!($($rest$(=$name_rest)?),+);
    };
}

// pub(crate) use set_empty_attr;

impl HtmlElement {
    set_attr!(
        accesskey,
        alt,
        class,
        contenteditable,
        decoding,
        dir,
        draggable,
        enterkeyhint,
        height,
        href,
        id,
        inputmode,
        lang,
        loading,
        max,
        maxlength,
        media,
        min,
        minlength,
        name,
        pattern,
        placeholder,
        rel,
        sizes,
        spellcheck,
        src,
        srcset,
        step,
        style,
        tabindex,
        target,
        title,
        translate,
        typ = "type",
        value,
        width
    );

    set_empty_attr!(
        autofocus, blocking, checked, defer, disabled, hidden, inert, multiple, nomodule, popover,
        r#async, readonly, required, selected
    );
}

// #[cfg(test)]
// mod test {
//     use crate::{elements::*, html_page::HtmlPage};
//
//     #[test]
//     fn render_simple() {
//         let page = HtmlPage::new()
//             .title("My page title")
//             .description("Some test page")
//             .add_body_child(h1().inner("A nice title"))
//             .add_body_child(div().add_child(p().inner("Some content...")))
//             .render_sorted();
//         insta::assert_snapshot!(page, @r#"
//         <!doctype html>
//         <html>
//           <head>
//             <title>My page title</title>
//             <meta charset="UTF-8">
//             <meta content="width=device-width, initial-scale=1.0, maximum-scale=1.0" name="viewport">
//             <meta content="Some test page" name="description">
//           </head>
//           <body>
//             <h1>A nice title</h1>
//             <div>
//               <p>Some content...</p>
//             </div>
//           </body>
//         </html>
//         "#);
//     }
//
//     #[test]
//     fn render_with_maybe_sets() {
//         let content = div()
//             .maybe_set_attr("class", || Some("mx-4"))
//             .maybe_set_empty_attr(|| Some("hidden"))
//             .maybe_set_empty_attr(|| -> Option<String> { None })
//             .maybe_add_child(|| Some(p().inner("yay")))
//             .render_sorted();
//         insta::assert_snapshot!(content, @r#"
//         <div class="mx-4" hidden>
//           <p>yay</p>
//         </div>
//         "#);
//     }
//
//     #[test]
//     fn render_multiple_inner() {
//         let content = p()
//             .inner("one")
//             .add_child(span().inner("two"))
//             .add_child(span().inner("three"))
//             .inner("four")
//             .add_child(span().inner("five"))
//             .inner("six")
//             .render_sorted();
//         insta::assert_snapshot!(content, @r"
//         <p>
//           one
//           <span>two</span>
//           <span>three</span>
//           four
//           <span>five</span>
//           six
//         </p>
//         ");
//     }
//
//     #[test]
//     fn add_remove_class_works() {
//         let content = div()
//             .class("flex mt-4")
//             .add_class("grid")
//             .add_class("flex-col")
//             .remove_class("grid")
//             .toogle_class("p-2")
//             .toogle_class("mt-4")
//             .render_sorted();
//         insta::assert_snapshot!(content, @r#"<div class="flex flex-col p-2"></div>"#);
//     }
//
//     #[test]
//     fn add_children_works() {
//         let content = div()
//             .add_children(
//                 (0..4)
//                     .map(|o| p().inner(o.to_string()).boxed())
//                     .collect::<Vec<_>>(),
//             )
//             .add_children(
//                 (4..8)
//                     .map(|o| p().inner(o.to_string()).boxed())
//                     .collect::<Vec<_>>(),
//             )
//             .render_sorted();
//         insta::assert_snapshot!(content, @r"
//         <div>
//           <p>0</p>
//           <p>1</p>
//           <p>2</p>
//           <p>3</p>
//           <p>4</p>
//           <p>5</p>
//           <p>6</p>
//           <p>7</p>
//         </div>
//         ");
//     }
// }
