use crate::{
    attribute::{AttributeValue, IntoAttributeValue},
    node::{HtmlNode, IntoNode, raw_node},
};
use indexmap::{IndexMap, IndexSet};
use std::borrow::Cow;

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
// pub fn have_attrs(&self) -> bool {
//     !(self.attrs.is_empty() && self.empty_attrs.is_empty())
// }

pub trait Element: Sized {
    fn tag(&self) -> &'static str;
    fn attrs(&self) -> &IndexMap<Cow<'static, str>, AttributeValue>;
    fn attrs_mut(&mut self) -> &mut IndexMap<Cow<'static, str>, AttributeValue>;
    fn classes(&self) -> &IndexSet<Cow<'static, str>>;
    fn classes_mut(&mut self) -> &mut IndexSet<Cow<'static, str>>;
    fn children(&self) -> &[HtmlNode];
    fn children_mut(&mut self) -> &mut Vec<HtmlNode>;
    fn is_void_tag(&self) -> bool;
    fn is_inline_tag(&self) -> bool;

    fn has_inline_content(&self) -> bool {
        let has_block = self.children().iter().any(|o| match o {
            HtmlNode::Element(x) => !x.is_inline_tag(),
            HtmlNode::SvgElement(x) => !x.is_inline_tag(),
            _ => false,
        });
        !has_block
    }

    fn set_attr<K, V>(mut self, attr: K, value: V) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        if let Some(v) = value.into_attr() {
            self.attrs_mut().insert(attr.into(), v);
            self
        } else {
            self
        }
    }

    fn set_raw_attr<K, V>(mut self, attr: K, value: V) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        if let Some(v) = value.into_raw_attr() {
            self.attrs_mut().insert(attr.into(), v);
            self
        } else {
            self
        }
    }

    fn set_empty_attr(mut self, attr: impl Into<Cow<'static, str>>) -> Self {
        self.attrs_mut().insert(attr.into(), AttributeValue::Empty);
        self
    }

    /// Sets the class attribute
    fn class(mut self, class: impl Into<Cow<'static, str>>) -> Self {
        if !self.classes().is_empty() {
            self.classes_mut().clear();
        }
        self.add_class(class)
    }

    fn has_class(&self, class: &str) -> bool {
        self.classes().contains(class)
    }

    fn add_class(mut self, class: impl Into<Cow<'static, str>>) -> Self {
        let class = class.into();
        if class.is_empty() {
            return self;
        }
        if class.contains(' ') {
            class.split_whitespace().for_each(|o| {
                self.classes_mut().insert(o.to_string().into());
            });
        } else {
            self.classes_mut().insert(class);
        }
        self
    }

    fn remove_class(mut self, class: &str) -> Self {
        self.classes_mut().shift_remove(class);
        self
    }

    fn toogle_class(self, class: impl Into<Cow<'static, str>>) -> Self {
        let class = class.into();
        if self.has_class(&class) {
            self.remove_class(&class)
        } else {
            self.add_class(class)
        }
    }

    /// Adds a raw html child
    fn add_raw(mut self, raw: impl ToString) -> Self {
        self.children_mut().push(raw_node(raw));
        self
    }

    /// Adds a child
    fn add_child(mut self, node: impl IntoNode) -> Self {
        let node = node.into_node();
        match node {
            HtmlNode::Fragment(mut x) => {
                self.children_mut().append(&mut x);
            }
            x => self.children_mut().push(x),
        }
        self
    }

    /// Alias for `add_child`
    #[inline]
    fn add(self, node: impl IntoNode) -> Self {
        self.add_child(node)
    }

    /// Add children
    fn add_children(mut self, nodes: impl IntoIterator<Item = impl IntoNode>) -> Self {
        self.children_mut()
            .extend(nodes.into_iter().map(|n| n.into_node()));
        self
    }
}

// TODO: simplify
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

pub(crate) use set_attr;

// TODO: simplify
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

pub(crate) use set_empty_attr;

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::render::Render;
//
//     #[test]
//     fn children_macro() {
//         let res = div()
//             .add_children(children!["Some text", p().add("inner text"), 123456])
//             .render();
//         insta::assert_snapshot!(res, @r"
//         <div>
//           Some text
//           <p>inner text</p>
//           123456
//         </div>
//         ");
//     }

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
