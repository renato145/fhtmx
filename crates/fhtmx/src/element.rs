use crate::{
    attribute::{AttributeValue, IntoAttributeValue},
    node::{HtmlNode, IntoNode, raw_node},
    prelude::HtmlElement,
};
use indexmap::{IndexMap, IndexSet};
use std::borrow::Cow;

/// Shared behavior for HTML and SVG elements.
///
/// Provides the builder methods used to compose attributes, classes and children.
/// All elements created by the tag functions in the prelude implement this trait.
///
/// # Examples
///
/// ```
/// use fhtmx::prelude::*;
///
/// let list = ul().add_children(["one", "two"].map(|x| li().add(x)));
/// assert_eq!(list.render(), "<ul>\n  <li>one</li>\n  <li>two</li>\n</ul>");
/// ```
pub trait Element: Sized {
    /// The element tag name, e.g. `"div"`.
    fn tag(&self) -> &'static str;
    /// Reference to the attribute map.
    fn attrs(&self) -> &IndexMap<Cow<'static, str>, AttributeValue>;
    /// Mutable reference to the attribute map.
    fn attrs_mut(&mut self) -> &mut IndexMap<Cow<'static, str>, AttributeValue>;
    /// Reference to the class set.
    fn classes(&self) -> &IndexSet<Cow<'static, str>>;
    /// Mutable reference to the class set.
    fn classes_mut(&mut self) -> &mut IndexSet<Cow<'static, str>>;
    /// Reference to child nodes.
    fn children(&self) -> &[HtmlNode];
    /// Mutable reference to child nodes.
    fn children_mut(&mut self) -> &mut Vec<HtmlNode>;
    /// Whether this is a void element (e.g. `<br>`, `<img>`).
    fn is_void_tag(&self) -> bool;
    /// Whether this is an inline element (affects indentation during rendering).
    fn is_inline_tag(&self) -> bool;

    /// Number of child nodes.
    #[inline]
    fn len(&self) -> usize {
        self.children().len()
    }

    /// Whether this element has no children.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the child at `index`.
    fn get_child(&self, index: usize) -> Option<&HtmlNode> {
        self.children().get(index)
    }

    /// Gets a mutable reference to the child at `index`.
    fn get_child_mut(&mut self, index: usize) -> Option<&mut HtmlNode> {
        self.children_mut().get_mut(index)
    }

    /// Whether all children are inline (no block-level children).
    fn has_inline_content(&self) -> bool {
        let has_block = self.children().iter().any(|o| match o {
            HtmlNode::Element(x) => !x.is_inline_tag(),
            HtmlNode::SvgElement(x) => !x.is_inline_tag(),
            _ => false,
        });
        !has_block
    }

    /// Sets an attribute on `self` mutably.
    fn set_attr_mut<K, V>(&mut self, attr: K, value: V)
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        if let Some(v) = value.into_attr() {
            self.attrs_mut().insert(attr.into(), v);
        }
    }

    /// Sets an attribute and returns `self`.
    fn set_attr<K, V>(mut self, attr: K, value: V) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        self.set_attr_mut(attr, value);
        self
    }

    /// Sets an attribute if `value` is `Some`.
    fn set_opt_attr_mut<K, V>(&mut self, attr: K, value: Option<V>)
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        if let Some(value) = value {
            self.set_attr_mut(attr, value);
        }
    }

    /// Sets an attribute if `value` is `Some` and returns `self`.
    fn set_opt_attr<K, V>(mut self, attr: K, value: Option<V>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        self.set_opt_attr_mut(attr, value);
        self
    }

    /// Sets a raw (unescaped) attribute on `self` mutably.
    fn set_raw_attr_mut<K, V>(&mut self, attr: K, value: V)
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        if let Some(v) = value.into_raw_attr() {
            self.attrs_mut().insert(attr.into(), v);
        }
    }

    /// Sets a raw (unescaped) attribute and returns `self`.
    fn set_raw_attr<K, V>(mut self, attr: K, value: V) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        self.set_raw_attr_mut(attr, value);
        self
    }

    /// Sets a raw attribute if `value` is `Some`.
    fn set_opt_raw_attr_mut<K, V>(&mut self, attr: K, value: Option<V>)
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        if let Some(value) = value {
            self.set_raw_attr_mut(attr, value);
        }
    }

    /// Sets a raw attribute if `value` is `Some` and returns `self`.
    fn set_opt_raw_attr<K, V>(mut self, attr: K, value: Option<V>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        self.set_opt_raw_attr_mut(attr, value);
        self
    }

    /// Sets a boolean (empty) attribute on `self` mutably.
    fn set_empty_attr_mut(&mut self, attr: impl Into<Cow<'static, str>>) {
        self.attrs_mut().insert(attr.into(), AttributeValue::Empty);
    }

    /// Sets a boolean (empty) attribute and returns `self`.
    fn set_empty_attr(mut self, attr: impl Into<Cow<'static, str>>) -> Self {
        self.set_empty_attr_mut(attr);
        self
    }

    /// Sets a boolean attribute if `attr` is `Some`.
    fn set_opt_empty_attr_mut(&mut self, attr: Option<impl Into<Cow<'static, str>>>) {
        if let Some(attr) = attr {
            self.set_empty_attr_mut(attr);
        }
    }

    /// Sets a boolean attribute if `attr` is `Some` and returns `self`.
    fn set_opt_empty_attr(mut self, attr: Option<impl Into<Cow<'static, str>>>) -> Self {
        self.set_opt_empty_attr_mut(attr);
        self
    }

    /// Replaces all classes with `class`.
    fn class_mut(&mut self, class: impl Into<Cow<'static, str>>) {
        if !self.classes().is_empty() {
            self.classes_mut().clear();
        }
        self.add_class_mut(class);
    }

    /// Replaces all classes with `class` and returns `self`.
    fn class(mut self, class: impl Into<Cow<'static, str>>) -> Self {
        self.class_mut(class);
        self
    }

    /// Whether the element has the given class.
    fn has_class(&self, class: &str) -> bool {
        self.classes().contains(class)
    }

    /// Adds a class to `self` mutably. Supports space-separated classes.
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

    /// Adds a class and returns `self`. Supports space-separated classes.
    fn add_class(mut self, class: impl Into<Cow<'static, str>>) -> Self {
        self.add_class_mut(class);
        self
    }

    /// Adds a class if `class` is `Some`.
    fn add_opt_class_mut(&mut self, class: Option<impl Into<Cow<'static, str>>>) {
        if let Some(class) = class {
            self.add_class_mut(class);
        }
    }

    /// Adds a class if `class` is `Some` and returns `self`.
    fn add_opt_class(mut self, class: Option<impl Into<Cow<'static, str>>>) -> Self {
        self.add_opt_class_mut(class);
        self
    }

    /// Removes a class from `self` mutably.
    fn remove_class_mut(&mut self, class: &str) {
        self.classes_mut().shift_remove(class);
    }

    /// Removes a class and returns `self`.
    fn remove_class(mut self, class: &str) -> Self {
        self.remove_class_mut(class);
        self
    }

    /// Toggles a class on `self` mutably.
    fn toggle_class_mut(&mut self, class: impl Into<Cow<'static, str>>) {
        let class = class.into();
        if self.has_class(&class) {
            self.remove_class_mut(&class)
        } else {
            self.add_class_mut(class)
        }
    }

    /// Toggles a class and returns `self`.
    fn toggle_class(mut self, class: impl Into<Cow<'static, str>>) -> Self {
        self.toggle_class_mut(class);
        self
    }

    /// Adds a raw HTML child mutably.
    fn add_raw_mut(&mut self, raw: impl ToString) {
        self.children_mut().push(raw_node(raw));
    }

    /// Adds a raw HTML child and returns `self`.
    fn add_raw(mut self, raw: impl ToString) -> Self {
        self.add_raw_mut(raw);
        self
    }

    /// Adds a child mutably.
    fn add_child_mut(&mut self, node: impl IntoNode) {
        let node = node.into_node();
        match node {
            HtmlNode::Fragment(mut x) => {
                self.children_mut().append(&mut x);
            }
            x => self.children_mut().push(x),
        }
    }

    /// Adds a child and returns `self`.
    fn add_child(mut self, node: impl IntoNode) -> Self {
        self.add_child_mut(node);
        self
    }

    /// Alias for [`add_child_mut`](Self::add_child_mut).
    #[inline]
    fn add_mut(&mut self, node: impl IntoNode) {
        self.add_child_mut(node)
    }

    /// Alias for [`add_child`](Self::add_child).
    #[inline]
    fn add(self, node: impl IntoNode) -> Self {
        self.add_child(node)
    }

    /// Adds a child if `node` is `Some`.
    fn add_opt_child_mut(&mut self, node: Option<impl IntoNode>) {
        if let Some(child) = node {
            self.add_child_mut(child);
        }
    }

    /// Adds a child if `node` is `Some` and returns `self`.
    fn add_opt_child(mut self, node: Option<impl IntoNode>) -> Self {
        self.add_opt_child_mut(node);
        self
    }

    /// Alias for [`add_opt_child_mut`](Self::add_opt_child_mut).
    #[inline]
    fn add_opt_mut(&mut self, node: Option<impl IntoNode>) {
        self.add_opt_child_mut(node)
    }

    /// Alias for [`add_opt_child`](Self::add_opt_child).
    #[inline]
    fn add_opt(self, node: Option<impl IntoNode>) -> Self {
        self.add_opt_child(node)
    }

    /// Inserts a child at `index` mutably.
    fn insert_child_mut(&mut self, index: usize, node: impl IntoNode) {
        let node = node.into_node();
        match node {
            HtmlNode::Fragment(x) => {
                self.children_mut().splice(index..index, x);
            }
            x => self.children_mut().insert(index, x),
        }
    }

    /// Inserts a child at `index` and returns `self`.
    fn insert_child(mut self, index: usize, node: impl IntoNode) -> Self {
        self.insert_child_mut(index, node);
        self
    }

    /// Inserts a child at `index` if `node` is `Some`.
    fn insert_opt_child_mut(&mut self, index: usize, node: Option<impl IntoNode>) {
        if let Some(child) = node {
            self.insert_child_mut(index, child);
        }
    }

    /// Inserts a child at `index` if `node` is `Some` and returns `self`.
    fn insert_opt_child(mut self, index: usize, node: Option<impl IntoNode>) -> Self {
        self.insert_opt_child_mut(index, node);
        self
    }

    /// Prepends a child mutably.
    fn prepend_child_mut(&mut self, node: impl IntoNode) {
        self.insert_child_mut(0, node)
    }

    /// Prepends a child and returns `self`.
    fn prepend_child(self, node: impl IntoNode) -> Self {
        self.insert_child(0, node)
    }

    /// Prepends a child if `node` is `Some`.
    fn prepend_opt_child_mut(&mut self, node: Option<impl IntoNode>) {
        self.insert_opt_child_mut(0, node)
    }

    /// Prepends a child if `node` is `Some` and returns `self`.
    fn prepend_opt_child(self, node: Option<impl IntoNode>) -> Self {
        self.insert_opt_child(0, node)
    }

    /// Adds multiple children mutably.
    fn add_children_mut(&mut self, nodes: impl IntoIterator<Item = impl IntoNode>) {
        self.children_mut()
            .extend(nodes.into_iter().map(|n| n.into_node()));
    }

    /// Adds multiple children and returns `self`.
    fn add_children(mut self, nodes: impl IntoIterator<Item = impl IntoNode>) -> Self {
        self.add_children_mut(nodes);
        self
    }

    /// Adds multiple children if `nodes` is `Some`.
    fn add_opt_children_mut(&mut self, nodes: Option<impl IntoIterator<Item = impl IntoNode>>) {
        if let Some(children) = nodes {
            self.add_children_mut(children);
        }
    }

    /// Adds multiple children if `nodes` is `Some` and returns `self`.
    fn add_opt_children(mut self, nodes: Option<impl IntoIterator<Item = impl IntoNode>>) -> Self {
        self.add_opt_children_mut(nodes);
        self
    }

    /// Inserts multiple children at `index` mutably.
    fn insert_children_mut(
        &mut self,
        index: usize,
        nodes: impl IntoIterator<Item = impl IntoNode>,
    ) {
        self.children_mut()
            .splice(index..index, nodes.into_iter().map(|o| o.into_node()));
    }

    /// Inserts multiple children at `index` and returns `self`.
    fn insert_children(
        mut self,
        index: usize,
        nodes: impl IntoIterator<Item = impl IntoNode>,
    ) -> Self {
        self.insert_children_mut(index, nodes);
        self
    }

    /// Inserts multiple children at `index` if `nodes` is `Some`.
    fn insert_opt_children_mut(
        &mut self,
        index: usize,
        nodes: Option<impl IntoIterator<Item = impl IntoNode>>,
    ) {
        if let Some(children) = nodes {
            self.insert_children_mut(index, children);
        }
    }

    /// Inserts multiple children at `index` if `nodes` is `Some` and returns `self`.
    fn insert_opt_children(
        mut self,
        index: usize,
        nodes: Option<impl IntoIterator<Item = impl IntoNode>>,
    ) -> Self {
        self.insert_opt_children_mut(index, nodes);
        self
    }

    /// Replaces the child at `index` with `f(child_element)` if it is an element.
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

    /// Replaces the child at `index` with `f(child_element)` if it is an element and returns `self`.
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
