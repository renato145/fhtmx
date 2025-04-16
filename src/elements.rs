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
pub struct HtmlLinkElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlScriptElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlStyleElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlInputElement;

pub trait HtmlRender: DynClone + Debug {
    fn render(&self) -> String {
        self.render_with_indent(0)
    }

    fn render_with_indent(&self, indent: usize) -> String;
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
}

#[derive(Debug, Clone, Copy)]
pub enum HtmlTagWrap {
    Wrap,
    NoWrap,
    /// https://developer.mozilla.org/en-US/docs/Glossary/Void_element
    Void,
}

#[derive(Clone)]
pub struct HtmlElement<T, G> {
    pub tag_name: T,
    pub attrs: HashMap<String, String>,
    pub empty_attrs: HashSet<String>,
    pub wrap_options: HtmlTagWrap,
    pub inner_text: Option<String>,
    pub children: Option<HtmlElements>,
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
            inner_text: None,
            children: None,
            group: PhantomData,
        }
    }

    /// Sets the inner text of the html element
    pub fn inner(mut self, inner_text: impl ToString) -> Self {
        self.inner_text = Some(inner_text.to_string());
        self
    }

    pub fn add_child<C: HtmlRender + 'static>(mut self, child: C) -> Self {
        match self.children.as_mut() {
            Some(children) => {
                children.push(Box::new(child));
            }
            None => {
                self.children = Some(vec![Box::new(child)]);
            }
        }
        self
    }

    pub fn add_children(mut self, mut children: HtmlElements) -> Self {
        match self.children.as_mut() {
            Some(current_children) => {
                current_children.append(&mut children);
            }
            None => {
                self.children = Some(children);
            }
        }
        self
    }

    pub fn set_attr(mut self, attr: impl ToString, value: impl ToString) -> Self {
        self.attrs.insert(attr.to_string(), value.to_string());
        self
    }

    pub fn set_empty_attr(mut self, attr: impl ToString) -> Self {
        self.empty_attrs.insert(attr.to_string());
        self
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

    pub fn render_children(&self) -> String {
        match &self.children {
            Some(children) => children.render(),
            None => String::new(),
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
        let tag = self.tag_name.as_ref();
        let (tag_start, tag_end) = match self.wrap_options {
            HtmlTagWrap::Wrap => (
                format!("<{}{}>", tag, self.render_attrs()),
                format!("</{}>", tag),
            ),
            HtmlTagWrap::NoWrap => (format!("<{}{} />", tag, self.render_attrs()), String::new()),
            HtmlTagWrap::Void => (format!("<{}{}>", tag, self.render_attrs()), String::new()),
        };
        let indent_str = "  ".repeat(indent);
        let inner = self.inner_text.clone().unwrap_or_default();
        match &self.children {
            Some(children) => {
                let inner = if inner.is_empty() {
                    inner
                } else {
                    format!("\n  {}{}", indent_str, inner)
                };
                format!(
                    "{}{}{}\n{}\n{}{}",
                    indent_str,
                    tag_start,
                    inner,
                    children.render_with_indent(indent + 1),
                    indent_str,
                    tag_end
                )
            }
            None => format!("{}{}{}{}", indent_str, tag_start, inner, tag_end),
        }
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
    HtmlTagWrap::Wrap => a, article, aside, audio, b, body, button, canvas, caption, code,
    colgroup, dd, details, div, dl, dt, em, fieldset, figcaption, figure, footer, form, h1, h2, h3,
    h4, h5, h6, head, header, html, i, iframe, label, legend, li, mark, math, nav, noscript,
    object, ol, option, p, pre, s, script:HtmlScriptElement, section, select, slot, small, span,
    strike, strong, style:HtmlStyleElement, sub, summary, sup, svg, table, tbody, td, template,
    textarea, tfoot, th, thead, title, tr, u, ul, video
);

create_web_element!(
    HtmlTagWrap::Void => area, base, br, col, embed, hr, img, input:HtmlInputElement, link:HtmlLinkElement, meta,
    param, source, track, wbr
);

pub fn main_tag() -> HtmlElement<&'static str, HtmlGenericElement> {
    HtmlElement::new("main", HtmlTagWrap::Wrap)
}

pub fn doctype_html() -> HtmlElement<&'static str, HtmlEmptyElement> {
    HtmlElement::new("!doctype", HtmlTagWrap::Void).set_empty_attr("html")
}

macro_rules! set_attr {
    ($attr:ident) => {
        paste! {
            #[doc = "Sets the `" $attr "` attribute."]
            pub fn $attr(self, value: impl ToString) -> Self {
                self.set_attr(stringify!([< $attr:lower >]), value)
            }
        }
    };

    ($attr:ident, $($rest:ident),+) => {
        set_attr!($attr);
        set_attr!($($rest),+);
    };
}

macro_rules! set_empty_attr {
    ($attr:ident) => {
        paste! {
            #[doc = "Sets the `" $attr "` empty attribute."]
            pub fn $attr(self) -> Self {
                self.set_empty_attr(stringify!([< $attr:lower >]))
            }
        }
    };

    ($attr:ident, $($rest:ident),+) => {
        set_empty_attr!($attr);
        set_empty_attr!($($rest),+);
    };
}

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

impl<T> HtmlElement<T, HtmlInputElement> {
    set_attr!(r#type, name, placeholder);
    set_empty_attr!(autofocus, required);
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
            .render();
        println!("{}", page);
    }
}
