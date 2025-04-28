use dyn_clone::DynClone;
use paste::paste;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    marker::PhantomData,
};

// Html element groups
#[derive(Debug, Clone, Copy)]
pub struct HtmlEmptyElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlGenericElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlAnchorElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlLinkElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlScriptElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlStyleElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlImgElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlInputElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlSelectElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlOptionElement;

pub trait HtmlRender: DynClone + Debug {
    fn render(&self) -> String {
        self.render_with_indent(0)
    }

    fn render_with_indent(&self, indent: usize) -> String;

    fn render_sorted(&self) -> String {
        self.render_sorted_with_indent(0)
    }

    fn render_sorted_with_indent(&self, indent: usize) -> String;
}

dyn_clone::clone_trait_object!(HtmlRender);

pub type HtmlSingleElement = Box<dyn HtmlRender>;
pub type HtmlElements = Vec<HtmlSingleElement>;

impl HtmlRender for HtmlElements {
    fn render_with_indent(&self, indent: usize) -> String {
        self.iter()
            .map(|o| o.render_with_indent(indent))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn render_sorted_with_indent(&self, indent: usize) -> String {
        self.iter()
            .map(|o| o.render_sorted_with_indent(indent))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HtmlTagWrap {
    Wrap,
    /// https://developer.mozilla.org/en-US/docs/Glossary/Void_element
    NoWrap,
}

#[derive(Clone)]
pub struct HtmlElement<T, G> {
    pub tag_name: T,
    pub attrs: HashMap<String, String>,
    pub empty_attrs: HashSet<String>,
    pub wrap_options: HtmlTagWrap,
    pub inner_text: Vec<(usize, String)>,
    pub children: Vec<(usize, HtmlSingleElement)>,
    content_idx: usize,
    group: PhantomData<G>,
}

impl<T: Debug, G> Debug for HtmlElement<T, G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HTMLElement")
            .field("tag_name", &self.tag_name)
            .field("attrs", &self.attrs)
            .field("empty_attrs", &self.empty_attrs)
            .field("wrap_options", &self.wrap_options)
            .field("inner_text", &self.inner_text)
            .field("children", &self.children)
            .field("group", &self.group)
            .finish()
    }
}

impl<T, G> HtmlElement<T, G> {
    pub fn new(tag_name: T, wrap_options: HtmlTagWrap) -> Self {
        HtmlElement {
            tag_name,
            attrs: HashMap::new(),
            empty_attrs: HashSet::new(),
            wrap_options,
            inner_text: Vec::new(),
            children: Vec::new(),
            content_idx: 0,
            group: PhantomData,
        }
    }

    /// Sets the inner text (html encoded) of the element
    pub fn inner(self, inner_text: &str) -> Self {
        self.inner_unsafe(htmlescape::encode_minimal(inner_text))
    }

    /// Sets the inner text (not html encoded) of the element
    pub fn inner_unsafe(mut self, inner_text: impl ToString) -> Self {
        self.inner_text
            .push((self.content_idx, inner_text.to_string()));
        self.content_idx += 1;
        self
    }

    pub fn add_child<C: HtmlRender + 'static>(mut self, child: C) -> Self {
        self.children.push((self.content_idx, Box::new(child)));
        self.content_idx += 1;
        self
    }

    /// Adds child if it contains a value
    pub fn add_opt_child<C: HtmlRender + 'static>(self, child: Option<C>) -> Self {
        if let Some(child) = child {
            return self.add_child(child);
        }
        self
    }

    /// Adds child if the closure returns a value
    pub fn maybe_add_child<F, C>(self, child: F) -> Self
    where
        F: FnOnce() -> Option<C>,
        C: HtmlRender + 'static,
    {
        self.add_opt_child(child())
    }

    pub fn add_children(mut self, children: HtmlElements) -> Self {
        let mut children = children
            .into_iter()
            .enumerate()
            .map(|(i, o)| (i + self.content_idx, o))
            .collect::<Vec<_>>();
        self.children.append(&mut children);
        self.content_idx += 1;
        self
    }

    /// Adds children if it contains a value
    pub fn add_opt_children(self, children: Option<HtmlElements>) -> Self {
        if let Some(children) = children {
            return self.add_children(children);
        }
        self
    }

    /// Adds children if the closure returns a value
    pub fn maybe_add_children<F>(self, children: F) -> Self
    where
        F: FnOnce() -> Option<HtmlElements>,
    {
        self.add_opt_children(children())
    }

    pub fn set_attr(mut self, attr: impl ToString, value: impl ToString) -> Self {
        self.attrs.insert(attr.to_string(), value.to_string());
        self
    }

    /// Sets an attribute if contains a value
    pub fn set_opt_attr(self, attr: impl ToString, value: Option<impl ToString>) -> Self {
        if let Some(value) = value {
            return self.set_attr(attr, value);
        }
        self
    }

    /// Sets an attribute if the closure returns a value
    pub fn maybe_set_attr<F, R>(self, attr: impl ToString, value: F) -> Self
    where
        F: FnOnce() -> Option<R>,
        R: ToString,
    {
        self.set_opt_attr(attr, value())
    }

    pub fn set_empty_attr(mut self, attr: impl ToString) -> Self {
        self.empty_attrs.insert(attr.to_string());
        self
    }

    /// Sets an empty_attrs if contains an attr
    pub fn set_opt_empty_attr(self, attr: Option<impl ToString>) -> Self {
        if let Some(attr) = attr {
            return self.set_empty_attr(attr);
        }
        self
    }

    /// Sets an empty attribute if the closure returns an attr
    pub fn maybe_set_empty_attr<F, R>(self, attr: F) -> Self
    where
        F: FnOnce() -> Option<R>,
        R: ToString,
    {
        self.set_opt_empty_attr(attr())
    }

    pub fn have_attrs(&self) -> bool {
        !(self.attrs.is_empty() && self.empty_attrs.is_empty())
    }

    pub fn render_attrs(&self) -> String {
        if !self.have_attrs() {
            return String::new();
        }
        let attrs = self
            .attrs
            .iter()
            .map(|(k, v)| format!("{}={:?}", k, v))
            .chain(self.empty_attrs.iter().cloned())
            .collect::<Vec<_>>()
            .join(" ");
        format!(" {}", attrs)
    }

    pub fn render_sorted_attrs(&self) -> String {
        if !self.have_attrs() {
            return String::new();
        }
        let mut attrs = self
            .attrs
            .iter()
            .map(|(k, v)| format!("{}={:?}", k, v))
            .chain(self.empty_attrs.iter().cloned())
            .collect::<Vec<_>>();
        attrs.sort();
        format!(" {}", attrs.join(" "))
    }
}

impl<T: AsRef<str>, G> HtmlElement<T, G> {
    fn _render(&self, indent: usize, sorted: bool) -> String {
        let tag = self.tag_name.as_ref();
        let attrs_str = if sorted {
            self.render_sorted_attrs()
        } else {
            self.render_attrs()
        };
        let (tag_start, tag_end) = match self.wrap_options {
            HtmlTagWrap::Wrap => (format!("<{}{}>", tag, attrs_str), format!("</{}>", tag)),
            HtmlTagWrap::NoWrap => (format!("<{}{} />", tag, attrs_str), String::new()),
        };
        let indent_str = "  ".repeat(indent);
        if self.children.is_empty() {
            let inner = self
                .inner_text
                .iter()
                .map(|(_, o)| o.clone())
                .collect::<Vec<_>>()
                .join(" ");
            format!("{}{}{}{}", indent_str, tag_start, inner, tag_end)
        } else {
            let mut children = self
                .children
                .iter()
                .map(|(i, o)| {
                    (
                        *i,
                        if sorted {
                            o.render_sorted_with_indent(indent + 1)
                        } else {
                            o.render_with_indent(indent + 1)
                        },
                    )
                })
                .chain(
                    self.inner_text
                        .iter()
                        .map(|(i, o)| (*i, format!("{}{}", "  ".repeat(indent + 1), o))),
                )
                .collect::<Vec<_>>();
            children.sort_by_key(|o| o.0);
            let inner = children
                .into_iter()
                .map(|o| o.1)
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "{}{}\n{}\n{}{}",
                indent_str, tag_start, inner, indent_str, tag_end
            )
        }
    }
}

impl<T: AsRef<str> + Debug + Clone + 'static, G: Clone + 'static> HtmlElement<T, G> {
    pub fn boxed(self) -> HtmlSingleElement {
        Box::new(self)
    }
}

impl<T: AsRef<str> + Debug + Clone, G: Clone> HtmlRender for HtmlElement<T, G> {
    fn render_with_indent(&self, indent: usize) -> String {
        self._render(indent, false)
    }

    fn render_sorted_with_indent(&self, indent: usize) -> String {
        self._render(indent, true)
    }
}

macro_rules! create_web_element {
    ($wrap:expr => $name:ident:$group:expr) => {
        paste! {
            #[doc = "Creates a `" $name "` html element."]
            pub fn $name() -> HtmlElement<&'static str, $group> {
                HtmlElement::new(stringify!($name), $wrap)
            }
        }
    };

    ($wrap:expr => $name:ident) => {
        create_web_element!($wrap => $name:HtmlGenericElement);
    };

    ($wrap:expr => $name:ident$(:$group:expr)?, $($rest:ident$(:$group_rest:expr)?),+) => {
        create_web_element!($wrap => $name$(:$group)?);
        create_web_element!($wrap => $($rest$(:$group_rest)?),+);
    };
}

create_web_element!(
    HtmlTagWrap::Wrap => a:HtmlAnchorElement, article, aside, audio, b, body, button, canvas,
    caption, code, colgroup, dd, details, div, dl, dt, em, fieldset, figcaption, figure, footer,
    form, h1, h2, h3, h4, h5, h6, head, header, html, i, iframe, label, legend, li, mark, math,
    nav, noscript, object, ol, option:HtmlOptionElement, p, pre, s, script:HtmlScriptElement,
    section, select:HtmlSelectElement, slot, small, span, strike, strong, style:HtmlStyleElement,
    sub, summary, sup, table, tbody, td, template, textarea, tfoot, th, thead, title, tr, u, ul,
    video
);

create_web_element!(
    HtmlTagWrap::NoWrap => area, base, br, col, embed, hr, img:HtmlImgElement, input:HtmlInputElement, link:HtmlLinkElement, meta,
    param, source, track, wbr
);

pub fn main_tag() -> HtmlElement<&'static str, HtmlGenericElement> {
    HtmlElement::new("main", HtmlTagWrap::Wrap)
}

pub fn doctype_html() -> HtmlElement<&'static str, HtmlEmptyElement> {
    HtmlElement::new("!doctype", HtmlTagWrap::NoWrap).set_empty_attr("html")
}

macro_rules! set_attr {
    ($attr:ident = $name:expr) => {
        paste! {
            #[doc = "Sets the `" $name "` attribute."]
            pub fn $attr(self, value: impl ToString) -> Self {
                self.set_attr($name, value)
            }
        }
    };

    ($attr:ident) => {
        paste! {
            #[doc = "Sets the `" $attr "` attribute."]
            pub fn $attr(self, value: impl ToString) -> Self {
                self.set_attr(stringify!([< $attr:lower >]), value)
            }
        }
    };

    ($attr:ident$(=$name:expr)?, $($rest:ident$(=$name_rest:expr)?),+) => {
        set_attr!($attr$(=$name)?);
        set_attr!($($rest$(=$name_rest)?),+);
    };
}

pub(crate) use set_attr;

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

impl<T, G> HtmlElement<T, G> {
    set_attr!(
        accesskey,
        class,
        contenteditable,
        dir,
        draggable,
        enterkeyhint,
        id,
        inputmode,
        lang,
        spellcheck,
        style,
        tabindex,
        title,
        translate
    );
    set_empty_attr!(hidden, inert, popover);
}

impl<T> HtmlElement<T, HtmlAnchorElement> {
    set_attr!(href, rel, target, r#type);
}

impl<T> HtmlElement<T, HtmlLinkElement> {
    set_attr!(href, rel, r#type);
}

impl<T> HtmlElement<T, HtmlScriptElement> {
    set_attr!(src, r#type);
    set_empty_attr!(r#async, defer, nomodule);
}

impl<T> HtmlElement<T, HtmlStyleElement> {
    set_attr!(media, r#type);
    set_empty_attr!(blocking);
}

impl<T> HtmlElement<T, HtmlImgElement> {
    set_attr!(alt, decoding, loading, sizes, src, srcset, height, width);
}

impl<T> HtmlElement<T, HtmlInputElement> {
    set_attr!(
        name,
        min,
        max,
        maxlength,
        min_length,
        pattern,
        placeholder,
        step,
        r#type,
        value
    );
    set_empty_attr!(autofocus, checked, disabled, readonly, required);
}

impl<T> HtmlElement<T, HtmlSelectElement> {
    set_attr!(name, placeholder, r#type, value);
    set_empty_attr!(autofocus, disabled, multiple, required);
}

impl<T> HtmlElement<T, HtmlOptionElement> {
    set_attr!(value);
    set_empty_attr!(disabled, selected);
}

#[cfg(test)]
mod test {
    use crate::{elements::*, html_page::HtmlPage};

    #[test]
    fn render_simple() {
        let page = HtmlPage::new()
            .title("My page title")
            .description("Some test page")
            .add_body_child(h1().inner("A nice title"))
            .add_body_child(div().add_child(p().inner("Some content...")))
            .render_sorted();
        println!("{}", page);
        insta::assert_yaml_snapshot!(page);
    }

    #[test]
    fn render_with_maybe_sets() {
        let content = div()
            .maybe_set_attr("class", || Some("mx-4"))
            .maybe_set_empty_attr(|| Some("hidden"))
            .maybe_set_empty_attr(|| -> Option<String> { None })
            .maybe_add_child(|| Some(p().inner("yay")))
            .render_sorted();
        println!("{}", content);
        insta::assert_yaml_snapshot!(content);
    }

    #[test]
    fn render_multiple_inner() {
        let content = p()
            .inner("one")
            .add_child(span().inner("two"))
            .add_child(span().inner("three"))
            .inner("four")
            .add_child(span().inner("five"))
            .inner("six")
            .render_sorted();
        println!("{}", content);
        insta::assert_yaml_snapshot!(content);
    }
}
