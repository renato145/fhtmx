use crate::{
    attribute::{AttributeValue, IntoAttributeValue},
    node::{HtmlNode, IntoNode, raw_node},
    prelude::HtmlElement,
};
use indexmap::{IndexMap, IndexSet};
use std::borrow::Cow;

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

    #[inline]
    fn len(&self) -> usize {
        self.children().len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the child at position
    fn get_child(&self, index: usize) -> Option<&HtmlNode> {
        self.children().get(index)
    }

    /// Gets the child at position
    fn get_child_mut(&mut self, index: usize) -> Option<&mut HtmlNode> {
        self.children_mut().get_mut(index)
    }

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

    /// Sets an attribute if contains a value
    fn set_opt_attr<K, V>(self, attr: K, value: Option<V>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        if let Some(value) = value {
            return self.set_attr(attr, value);
        }
        self
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

    /// Sets an attribute if contains a value
    fn set_opt_raw_attr<K, V>(self, attr: K, value: Option<V>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        if let Some(value) = value {
            return self.set_raw_attr(attr, value);
        }
        self
    }

    fn set_empty_attr(mut self, attr: impl Into<Cow<'static, str>>) -> Self {
        self.attrs_mut().insert(attr.into(), AttributeValue::Empty);
        self
    }

    /// Sets an empty_attrs if contains an attr
    fn set_opt_empty_attr(self, attr: Option<impl Into<Cow<'static, str>>>) -> Self {
        if let Some(attr) = attr {
            return self.set_empty_attr(attr);
        }
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

    fn add_opt_class(self, class: Option<impl Into<Cow<'static, str>>>) -> Self {
        if let Some(class) = class {
            self.add_class(class)
        } else {
            self
        }
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

    /// Adds child if it contains a value
    fn add_opt_child(self, node: Option<impl IntoNode>) -> Self {
        if let Some(child) = node {
            return self.add_child(child);
        }
        self
    }

    /// Alias for `add_opt_child`
    #[inline]
    fn add_opt(self, node: Option<impl IntoNode>) -> Self {
        self.add_opt_child(node)
    }

    fn insert_child(mut self, index: usize, node: impl IntoNode) -> Self {
        let node = node.into_node();
        match node {
            HtmlNode::Fragment(x) => {
                self.children_mut().splice(index..index, x);
            }
            x => self.children_mut().insert(index, x),
        }
        self
    }

    /// Adds child if it contains a value
    fn insert_opt_child(self, index: usize, node: Option<impl IntoNode>) -> Self {
        if let Some(child) = node {
            return self.insert_child(index, child);
        }
        self
    }

    fn prepend_child(self, node: impl IntoNode) -> Self {
        self.insert_child(0, node)
    }

    fn prepend_opt_child(self, node: Option<impl IntoNode>) -> Self {
        self.insert_opt_child(0, node)
    }

    /// Add children
    fn add_children(mut self, nodes: impl IntoIterator<Item = impl IntoNode>) -> Self {
        self.children_mut()
            .extend(nodes.into_iter().map(|n| n.into_node()));
        self
    }

    /// Adds children if it contains a value
    fn add_opt_children(self, nodes: Option<impl IntoIterator<Item = impl IntoNode>>) -> Self {
        if let Some(children) = nodes {
            return self.add_children(children);
        }
        self
    }

    /// Add children
    fn insert_children(
        mut self,
        index: usize,
        nodes: impl IntoIterator<Item = impl IntoNode>,
    ) -> Self {
        self.children_mut()
            .splice(index..index, nodes.into_iter().map(|o| o.into_node()));
        self
    }

    /// Adds child if it contains a value
    fn insert_opt_children(
        self,
        index: usize,
        nodes: Option<impl IntoIterator<Item = impl IntoNode>>,
    ) -> Self {
        if let Some(children) = nodes {
            return self.insert_children(index, children);
        }
        self
    }

    fn update_html_element<F>(mut self, index: usize, f: F) -> Self
    where
        F: FnOnce(HtmlElement) -> HtmlElement,
    {
        if index >= self.len() {
            return self;
        }
        let node = match self.children_mut().remove(index) {
            HtmlNode::Element(el) => f(el).into_node(),
            x => x,
        };
        self.insert_child(index, node)
    }
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{html_element::*, render::Render};

    #[test]
    fn render_with_maybe_sets() {
        let res = div()
            .add_opt_class(Some("mx-4"))
            .set_opt_empty_attr(Some("hidden"))
            .set_opt_empty_attr(None::<&str>)
            .add_opt(Some("yay"))
            .render();
        insta::assert_snapshot!(res, @r#"<div class="mx-4" hidden>yay</div>"#);
    }

    #[test]
    fn add_remove_class_works() {
        let res = div()
            .class("flex mt-4")
            .add_class("grid")
            .add_class("flex-col")
            .remove_class("grid")
            .toogle_class("p-2")
            .toogle_class("mt-4")
            .render();
        insta::assert_snapshot!(res, @r#"<div class="flex flex-col p-2"></div>"#);
    }

    #[test]
    fn add_children_works() {
        let res = div().add_children('a'..'f').render();
        insta::assert_snapshot!(res, @"<div>abcde</div>");
    }

    #[test]
    fn update_html_element_works() {
        let res = div()
            .add(div().add(p().add("First")).add(p().add("Third")))
            .add("Another content");
        let res = res
            .update_html_element(0, |x| x.insert_child(1, p().add("Second")))
            .render();
        insta::assert_snapshot!(res, @r"
        <div>
          <div>
            <p>First</p>
            <p>Second</p>
            <p>Third</p>
          </div>
          Another content
        </div>
        ");
    }
}
