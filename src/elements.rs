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
pub struct HtmlInputElement;

pub trait HtmlRender: DynClone + Debug {
    fn render(&self) -> String {
        self.render_with_indent(false)
    }

    fn render_with_indent(&self, indent: bool) -> String;
}

dyn_clone::clone_trait_object!(HtmlRender);

pub type HtmlElements = Vec<Box<dyn HtmlRender>>;

impl HtmlRender for HtmlElements {
    fn render_with_indent(&self, indent: bool) -> String {
        self.iter()
            .map(|o| {
                o.render_with_indent(indent)
                    .lines()
                    .map(|o| {
                        if indent {
                            format!("  {}", o)
                        } else {
                            o.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HtmlTagWrap {
    Wrap,
    NoWrap,
    NoWrapNoClose,
}

#[derive(Clone)]
pub struct HTMLElement<T, G> {
    pub tag_name: T,
    pub attrs: HashMap<String, String>,
    pub empty_attrs: HashSet<String>,
    pub wrap_options: HtmlTagWrap,
    pub children: Option<HtmlElements>,
    group: PhantomData<G>,
}

impl<T: Debug, G> Debug for HTMLElement<T, G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HTMLElement")
            .field("tag_name", &self.tag_name)
            .field("attrs", &self.attrs)
            .field("empty_attrs", &self.empty_attrs)
            .field("wrap_options", &self.wrap_options)
            .field("children", &self.children)
            .field("group", &self.group)
            .finish()
    }
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

impl<T, G> HTMLElement<T, G> {
    pub fn new(tag_name: T, wrap_options: HtmlTagWrap) -> Self {
        HTMLElement {
            tag_name,
            attrs: HashMap::new(),
            empty_attrs: HashSet::new(),
            wrap_options,
            children: None,
            group: PhantomData,
        }
    }

    pub fn add_child<Q: AsRef<str> + Debug + Clone + 'static, W: Clone + 'static>(
        mut self,
        child: HTMLElement<Q, W>,
    ) -> Self {
        match self.children.as_mut() {
            Some(children) => {
                children.push(child.boxed());
            }
            None => {
                self.children = Some(vec![child.boxed()]);
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

    set_attr!(id, class, title, style);
    set_empty_attr!(hidden, autofocus);
}

impl<T> HTMLElement<T, HtmlInputElement> {
    set_attr!(r#type, name);
}

impl<T: AsRef<str> + Debug + Clone + 'static, G: Clone + 'static> HTMLElement<T, G> {
    pub fn boxed(self) -> Box<dyn HtmlRender> {
        Box::new(self)
    }
}

impl<T: AsRef<str> + Debug + Clone, G: Clone> HtmlRender for HTMLElement<T, G> {
    fn render_with_indent(&self, _indent: bool) -> String {
        let tag = self.tag_name.as_ref();
        let (tag_start, tag_end) = match self.wrap_options {
            HtmlTagWrap::Wrap => (
                format!("<{}{}>", tag, self.render_attrs()),
                format!("</{}>", tag),
            ),
            HtmlTagWrap::NoWrap => (format!("<{}{} />", tag, self.render_attrs()), String::new()),
            HtmlTagWrap::NoWrapNoClose => {
                (format!("<{}{}>", tag, self.render_attrs()), String::new())
            }
        };
        match &self.children {
            Some(children) => format!(
                "{}\n{}\n{}",
                tag_start,
                children.render_with_indent(true),
                tag_end
            ),
            None => format!("{}{}", tag_start, tag_end),
        }
    }
}

pub fn doctype_html() -> HTMLElement<&'static str, HtmlEmptyElement> {
    HTMLElement::new("!doctype", HtmlTagWrap::NoWrapNoClose).set_empty_attr("html")
}

macro_rules! create_web_element {
    ($name:ident) => {
        paste! {
            #[doc = "Creates a `" $name "` html element."]
            pub fn $name() -> HTMLElement<&'static str, HtmlGenericElement> {
                HTMLElement::new(stringify!($name), HtmlTagWrap::Wrap)
            }
        }
    };

    ($name:ident, $($rest:ident),+) => {
        create_web_element!($name);
        create_web_element!($($rest),+);
    };
}

create_web_element!(
    html, head, title, meta, style, body, pre, code, div, span, p, h1, h2, h3, h4, h5, h6, strong,
    em, b, i, u, s, strike, sub, sup, hr, br, img, a, link, nav, ul, ol, li, dl, dt, dd, table,
    thead, tbody, tfoot, tr, th, td, caption, col, colgroup, form, textarea, button, select,
    option, label, fieldset, legend, details, summary, main, header, footer, section, article,
    aside, figure, figcaption, mark, small, iframe, object, embed, param, video, audio, source,
    canvas, svg, math, script, noscript, template, slot
);

pub fn input() -> HTMLElement<&'static str, HtmlInputElement> {
    HTMLElement::new("input", HtmlTagWrap::NoWrap)
}

// voids = set('area base br col command embed hr img input keygen link meta param source track wbr !doctype'.split())
// _g = globals()

// _block_tags = {'div', 'p', 'ul', 'ol', 'li', 'table', 'thead', 'tbody', 'tfoot',
//                'html', 'head', 'body', 'meta', 'title', '!doctype', 'input', 'script', 'link', 'style',
//                'tr', 'th', 'td', 'section', 'article', 'nav', 'aside', 'header',
//                'footer', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'blockquote'}
// _inline_tags = {'a', 'span', 'b', 'i', 'u', 'em', 'strong', 'img', 'br', 'small',
//                 'big', 'sub', 'sup', 'label', 'input', 'select', 'option'}

// html_attrs = 'id cls title style accesskey contenteditable dir draggable enterkeyhint hidden inert inputmode lang popover spellcheck tabindex translate'.split()
// hx_attrs = 'get post put delete patch trigger target swap swap_oob include select select_oob indicator push_url confirm disable replace_url vals disabled_elt ext headers history history_elt indicator inherit params preserve prompt replace_url request sync validate'
// hx_attrs = [f'hx_{o}' for o in hx_attrs.split()]
// hx_attrs_annotations = {
//     "hx_swap": Literal["innerHTML", "outerHTML", "afterbegin", "beforebegin", "beforeend", "afterend", "delete", "none"] | str,
//     "hx_swap_oob": Literal["true", "innerHTML", "outerHTML", "afterbegin", "beforebegin", "beforeend", "afterend", "delete", "none"] | str,
//     "hx_push_url": Literal["true", "false"] | str,
//     "hx_replace_url": Literal["true", "false"] | str,
//     "hx_disabled_elt": Literal["this", "next", "previous"] | str,
//     "hx_history": Literal["false"] | str,
//     "hx_params": Literal["*", "none"] | str,
//     "hx_validate": Literal["true", "false"],

#[cfg(test)]
mod test {
    use crate::elements::{HtmlRender, body, div, doctype_html, form, head, html, input};

    #[test]
    fn render_simple() {
        let res = vec![
            doctype_html().boxed(),
            html()
                .add_child(head())
                .add_child(
                    body()
                        .add_children(vec![div().id("x").boxed(), div().id("y").boxed()])
                        .add_child(form().add_child(input().r#type("text").name("user"))),
                )
                .boxed(),
        ]
        .render();
        println!("{}", res);
    }
}
