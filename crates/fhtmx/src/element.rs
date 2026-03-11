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

    fn set_attr_mut<K, V>(&mut self, attr: K, value: V)
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        if let Some(v) = value.into_attr() {
            self.attrs_mut().insert(attr.into(), v);
        }
    }

    fn set_attr<K, V>(mut self, attr: K, value: V) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        self.set_attr_mut(attr, value);
        self
    }

    /// Sets an attribute if contains a value
    fn set_opt_attr_mut<K, V>(&mut self, attr: K, value: Option<V>)
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        if let Some(value) = value {
            self.set_attr_mut(attr, value);
        }
    }

    /// Sets an attribute if contains a value
    fn set_opt_attr<K, V>(mut self, attr: K, value: Option<V>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        self.set_opt_attr_mut(attr, value);
        self
    }

    fn set_raw_attr_mut<K, V>(&mut self, attr: K, value: V)
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        if let Some(v) = value.into_raw_attr() {
            self.attrs_mut().insert(attr.into(), v);
        }
    }

    fn set_raw_attr<K, V>(mut self, attr: K, value: V) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        self.set_raw_attr_mut(attr, value);
        self
    }

    /// Sets an attribute if contains a value
    fn set_opt_raw_attr_mut<K, V>(&mut self, attr: K, value: Option<V>)
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        if let Some(value) = value {
            self.set_raw_attr_mut(attr, value);
        }
    }

    /// Sets an attribute if contains a value
    fn set_opt_raw_attr<K, V>(mut self, attr: K, value: Option<V>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        self.set_opt_raw_attr_mut(attr, value);
        self
    }

    fn set_empty_attr_mut(&mut self, attr: impl Into<Cow<'static, str>>) {
        self.attrs_mut().insert(attr.into(), AttributeValue::Empty);
    }

    fn set_empty_attr(mut self, attr: impl Into<Cow<'static, str>>) -> Self {
        self.set_empty_attr_mut(attr);
        self
    }

    /// Sets an empty_attrs if contains an attr
    fn set_opt_empty_attr_mut(&mut self, attr: Option<impl Into<Cow<'static, str>>>) {
        if let Some(attr) = attr {
            self.set_empty_attr_mut(attr);
        }
    }

    /// Sets an empty_attrs if contains an attr
    fn set_opt_empty_attr(mut self, attr: Option<impl Into<Cow<'static, str>>>) -> Self {
        self.set_opt_empty_attr_mut(attr);
        self
    }

    /// Sets the class attribute
    fn class_mut(&mut self, class: impl Into<Cow<'static, str>>) {
        if !self.classes().is_empty() {
            self.classes_mut().clear();
        }
        self.add_class_mut(class);
    }

    /// Sets the class attribute
    fn class(mut self, class: impl Into<Cow<'static, str>>) -> Self {
        self.class_mut(class);
        self
    }

    fn has_class(&self, class: &str) -> bool {
        self.classes().contains(class)
    }

    fn add_class_mut(&mut self, class: impl Into<Cow<'static, str>>) {
        let class = class.into();
        if class.is_empty() {
            return;
        }
        if class.contains(' ') {
            class.split_whitespace().for_each(|o| {
                self.classes_mut().insert(o.to_string().into());
            });
        } else {
            self.classes_mut().insert(class);
        }
    }

    fn add_class(mut self, class: impl Into<Cow<'static, str>>) -> Self {
        self.add_class_mut(class);
        self
    }

    fn add_opt_class_mut(&mut self, class: Option<impl Into<Cow<'static, str>>>) {
        if let Some(class) = class {
            self.add_class_mut(class);
        }
    }

    fn add_opt_class(mut self, class: Option<impl Into<Cow<'static, str>>>) -> Self {
        self.add_opt_class_mut(class);
        self
    }

    fn remove_class_mut(&mut self, class: &str) {
        self.classes_mut().shift_remove(class);
    }

    fn remove_class(mut self, class: &str) -> Self {
        self.remove_class_mut(class);
        self
    }

    fn toggle_class_mut(&mut self, class: impl Into<Cow<'static, str>>) {
        let class = class.into();
        if self.has_class(&class) {
            self.remove_class_mut(&class)
        } else {
            self.add_class_mut(class)
        }
    }

    fn toggle_class(mut self, class: impl Into<Cow<'static, str>>) -> Self {
        self.toggle_class_mut(class);
        self
    }

    /// Adds a raw html child
    fn add_raw_mut(&mut self, raw: impl ToString) {
        self.children_mut().push(raw_node(raw));
    }

    /// Adds a raw html child
    fn add_raw(mut self, raw: impl ToString) -> Self {
        self.add_raw_mut(raw);
        self
    }

    /// Adds a child
    fn add_child_mut(&mut self, node: impl IntoNode) {
        let node = node.into_node();
        match node {
            HtmlNode::Fragment(mut x) => {
                self.children_mut().append(&mut x);
            }
            x => self.children_mut().push(x),
        }
    }

    /// Adds a child
    fn add_child(mut self, node: impl IntoNode) -> Self {
        self.add_child_mut(node);
        self
    }

    /// Alias for `add_child_mut`
    #[inline]
    fn add_mut(&mut self, node: impl IntoNode) {
        self.add_child_mut(node)
    }

    /// Alias for `add_child`
    #[inline]
    fn add(self, node: impl IntoNode) -> Self {
        self.add_child(node)
    }

    /// Adds child if it contains a value
    fn add_opt_child_mut(&mut self, node: Option<impl IntoNode>) {
        if let Some(child) = node {
            self.add_child_mut(child);
        }
    }

    /// Adds child if it contains a value
    fn add_opt_child(mut self, node: Option<impl IntoNode>) -> Self {
        self.add_opt_child_mut(node);
        self
    }

    /// Alias for `add_opt_child_mut`
    #[inline]
    fn add_opt_mut(&mut self, node: Option<impl IntoNode>) {
        self.add_opt_child_mut(node)
    }

    /// Alias for `add_opt_child`
    #[inline]
    fn add_opt(self, node: Option<impl IntoNode>) -> Self {
        self.add_opt_child(node)
    }

    fn insert_child_mut(&mut self, index: usize, node: impl IntoNode) {
        let node = node.into_node();
        match node {
            HtmlNode::Fragment(x) => {
                self.children_mut().splice(index..index, x);
            }
            x => self.children_mut().insert(index, x),
        }
    }

    fn insert_child(mut self, index: usize, node: impl IntoNode) -> Self {
        self.insert_child_mut(index, node);
        self
    }

    /// Adds child if it contains a value
    fn insert_opt_child_mut(&mut self, index: usize, node: Option<impl IntoNode>) {
        if let Some(child) = node {
            self.insert_child_mut(index, child);
        }
    }

    /// Adds child if it contains a value
    fn insert_opt_child(mut self, index: usize, node: Option<impl IntoNode>) -> Self {
        self.insert_opt_child_mut(index, node);
        self
    }

    fn prepend_child_mut(&mut self, node: impl IntoNode) {
        self.insert_child_mut(0, node)
    }

    fn prepend_child(self, node: impl IntoNode) -> Self {
        self.insert_child(0, node)
    }

    fn prepend_opt_child_mut(&mut self, node: Option<impl IntoNode>) {
        self.insert_opt_child_mut(0, node)
    }

    fn prepend_opt_child(self, node: Option<impl IntoNode>) -> Self {
        self.insert_opt_child(0, node)
    }

    /// Add children
    fn add_children_mut(&mut self, nodes: impl IntoIterator<Item = impl IntoNode>) {
        self.children_mut()
            .extend(nodes.into_iter().map(|n| n.into_node()));
    }

    /// Add children
    fn add_children(mut self, nodes: impl IntoIterator<Item = impl IntoNode>) -> Self {
        self.add_children_mut(nodes);
        self
    }

    /// Adds children if it contains a value
    fn add_opt_children_mut(&mut self, nodes: Option<impl IntoIterator<Item = impl IntoNode>>) {
        if let Some(children) = nodes {
            self.add_children_mut(children);
        }
    }

    /// Adds children if it contains a value
    fn add_opt_children(mut self, nodes: Option<impl IntoIterator<Item = impl IntoNode>>) -> Self {
        self.add_opt_children_mut(nodes);
        self
    }

    /// Add children
    fn insert_children_mut(
        &mut self,
        index: usize,
        nodes: impl IntoIterator<Item = impl IntoNode>,
    ) {
        self.children_mut()
            .splice(index..index, nodes.into_iter().map(|o| o.into_node()));
    }

    /// Add children
    fn insert_children(
        mut self,
        index: usize,
        nodes: impl IntoIterator<Item = impl IntoNode>,
    ) -> Self {
        self.insert_children_mut(index, nodes);
        self
    }

    /// Adds child if it contains a value
    fn insert_opt_children_mut(
        &mut self,
        index: usize,
        nodes: Option<impl IntoIterator<Item = impl IntoNode>>,
    ) {
        if let Some(children) = nodes {
            self.insert_children_mut(index, children);
        }
    }

    /// Adds child if it contains a value
    fn insert_opt_children(
        mut self,
        index: usize,
        nodes: Option<impl IntoIterator<Item = impl IntoNode>>,
    ) -> Self {
        self.insert_opt_children_mut(index, nodes);
        self
    }

    fn update_html_element_mut<F>(&mut self, index: usize, f: F)
    where
        F: FnOnce(HtmlElement) -> HtmlElement,
    {
        if index >= self.len() {
            return;
        }
        let node = match self.children_mut().remove(index) {
            HtmlNode::Element(el) => f(el).into_node(),
            x => x,
        };
        self.insert_child_mut(index, node);
    }

    fn update_html_element<F>(mut self, index: usize, f: F) -> Self
    where
        F: FnOnce(HtmlElement) -> HtmlElement,
    {
        self.update_html_element_mut(index, f);
        self
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
            .toggle_class("p-2")
            .toggle_class("mt-4")
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
