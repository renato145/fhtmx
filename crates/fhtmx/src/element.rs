use crate::{
    attribute::{AttributeValue, IntoAttributeValue},
    node::{HtmlNode, IntoNode, raw_node},
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

pub trait Element: Sized {
    fn tag(&self) -> &'static str;
    fn attrs(&self) -> &IndexMap<Cow<'static, str>, AttributeValue>;
    fn children(&self) -> &[HtmlNode];
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

    /// Adds a raw html child
    fn add_raw(self, raw: impl ToString) -> Self;

    /// Adds a child
    fn add_child(self, node: impl IntoNode) -> Self;

    /// Add children
    fn add_children(self, nodes: impl IntoIterator<Item = impl IntoNode>) -> Self;

    fn insert_attr(self, attr: impl Into<Cow<'static, str>>, value: AttributeValue) -> Self;

    fn set_attr<K, V>(self, attr: K, value: V) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        if let Some(v) = value.into_attr() {
            self.insert_attr(attr, v)
        } else {
            self
        }
    }

    fn set_raw_attr<K, V>(self, attr: K, value: V) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: IntoAttributeValue,
    {
        if let Some(v) = value.into_raw_attr() {
            self.insert_attr(attr.into(), v)
        } else {
            self
        }
    }

    fn set_empty_attr(self, attr: impl Into<Cow<'static, str>>) -> Self {
        self.insert_attr(attr.into(), AttributeValue::Empty)
    }
}

impl Element for HtmlElement {
    #[inline]
    fn tag(&self) -> &'static str {
        self.tag
    }

    #[inline]
    fn attrs(&self) -> &IndexMap<Cow<'static, str>, AttributeValue> {
        &self.attrs
    }

    #[inline]
    fn children(&self) -> &[HtmlNode] {
        &self.children
    }

    #[inline]
    fn is_void_tag(&self) -> bool {
        VOID_ELEMENTS.contains(&self.tag())
    }

    #[inline]
    fn is_inline_tag(&self) -> bool {
        INLINE_ELEMENTS.contains(&self.tag())
    }

    fn add_raw(mut self, raw: impl ToString) -> Self {
        self.children.push(raw_node(raw));
        self
    }

    fn add_child(mut self, node: impl IntoNode) -> Self {
        let node = node.into_node();
        match node {
            HtmlNode::Fragment(mut x) => {
                self.children.append(&mut x);
            }
            x => self.children.push(x),
        }
        self
    }

    fn add_children(mut self, nodes: impl IntoIterator<Item = impl IntoNode>) -> Self {
        self.children
            .extend(nodes.into_iter().map(|n| n.into_node()));
        self
    }

    fn insert_attr(mut self, attr: impl Into<Cow<'static, str>>, value: AttributeValue) -> Self {
        self.attrs.insert(attr.into(), value);
        self
    }
}

macro_rules! create_tag_fn {
    ($name:ident) => {
        paste! {
            #[doc = "Creates a `" $name "` html element."]
            pub fn $name() -> HtmlElement {
                HtmlElement::new(stringify!($name))
            }
        }
    };

    ($name:ident; $eg:expr) => {
        paste! {
            #[doc = "Creates a `" $name "` html element.\n"$eg]
            pub fn $name() -> HtmlElement {
                HtmlElement::new(stringify!($name))
            }
        }
    };

    ($name:ident$(;$eg:expr)?, $($rest:ident$(;$eg_rest:expr)?),+) => {
        create_tag_fn!($name$(;$eg)?);
        create_tag_fn!($($rest$(;$eg_rest)?),+);
    };
}

create_tag_fn!(
    a; "Defines a hyperlink",
    abbr; "Defines an abbreviation or an acronym",
    address; "Defines contact information for the author/owner of a document",
    area; "Defines an area inside an image map",
    article; "Defines an article",
    aside; "Defines content aside from the page content",
    audio; "Defines embedded sound content",
    b; "Defines bold text",
    base; "Specifies the base URL/target for all relative URLs in a document",
    bdi; "Isolates a part of text that might be formatted in a different direction from other text outside it",
    bdo; "Overrides the current text direction",
    blockquote; "Defines a section that is quoted from another source",
    body; "Defines the document's body",
    br; "Defines a single line break",
    button; "Defines a clickable button",
    canvas; "Used to draw graphics, on the fly, via scripting (usually JavaScript)",
    caption; "Defines a table caption",
    cite; "Defines the title of a work",
    code; "Defines a piece of computer code",
    col; "Specifies column properties for each column within a <colgroup> element",
    colgroup; "Specifies a group of one or more columns in a table for formatting",
    data; "Adds a machine-readable translation of a given content",
    datalist; "Specifies a list of pre-defined options for input controls",
    dd; "Defines a description/value of a term in a description list",
    del; "Defines text that has been deleted from a document",
    details; "Defines additional details that the user can view or hide",
    dfn; "Specifies a term that is going to be defined within the content",
    dialog; "Defines a dialog box or window",
    div; "Defines a section in a document",
    dl; "Defines a description list",
    dt; "Defines a term/name in a description list",
    em; "Defines emphasized text",
    embed; "Defines a container for an external application",
    fieldset; "Groups related elements in a form",
    figcaption; "Defines a caption for a <figure> element",
    figure; "Specifies self-contained content",
    footer; "Defines a footer for a document or section",
    form; "Defines an HTML form for user input",
    h1; "Defines HTML headings",
    h2; "Defines HTML headings",
    h3; "Defines HTML headings",
    h4; "Defines HTML headings",
    h5; "Defines HTML headings",
    h6; "Defines HTML headings",
    head; "Contains metadata/information for the document",
    header; "Defines a header for a document or section",
    hgroup; "Defines a header and related content",
    hr; "Defines a thematic change in the content",
    html; "Defines the root of an HTML document",
    i; "Defines a part of text in an alternate voice or mood",
    iframe; "Defines an inline frame",
    img; "Defines an image",
    input; "Defines an input control",
    ins; "Defines a text that has been inserted into a document",
    kbd; "Defines keyboard input",
    label; "Defines a label for an <input> element",
    legend; "Defines a caption for a <fieldset> element",
    li; "Defines a list item",
    link; "Defines the relationship between a document and an external resource (most used to link to style sheets)",
    map; "Defines an image map",
    mark; "Defines marked/highlighted text",
    menu; "Defines an unordered list",
    meta; "Defines metadata about an HTML document",
    meter; "Defines a scalar measurement within a known range (a gauge)",
    nav; "Defines navigation links",
    noscript; "Defines an alternate content for users that do not support client-side scripts",
    object; "Defines a container for an external application",
    ol; "Defines an ordered list",
    optgroup; "Defines a group of related options in a drop-down list",
    option; "Defines an option in a drop-down list",
    output; "Defines the result of a calculation",
    p; "Defines a paragraph",
    param; "Defines a parameter for an object",
    picture; "Defines a container for multiple image resources",
    pre; "Defines preformatted text",
    progress; "Represents the progress of a task",
    q; "Defines a short quotation",
    rp; "Defines what to show in browsers that do not support ruby annotations",
    rt; "Defines an explanation/pronunciation of characters (for East Asian typography)",
    ruby; "Defines a ruby annotation (for East Asian typography)",
    s; "Defines text that is no longer correct",
    samp; "Defines sample output from a computer program",
    script; "Defines a client-side script",
    search; "Defines a search section",
    section; "Defines a section in a document",
    select; "Defines a drop-down list",
    small; "Defines smaller text",
    source; "Defines multiple media resources for media elements (<video> and <audio>)",
    span; "Defines a section in a document",
    strong; "Defines important text",
    style; "Defines style information for a document",
    sub; "Defines subscripted text",
    summary; "Defines a visible heading for a <details> element",
    sup; "Defines superscripted text",
    table; "Defines a table",
    tbody; "Groups the body content in a table",
    td; "Defines a cell in a table",
    template; "Defines a container for content that should be hidden when the page loads",
    textarea; "Defines a multiline input control (text area)",
    tfoot; "Groups the footer content in a table",
    th; "Defines a header cell in a table",
    thead; "Groups the header content in a table",
    time; "Defines a specific time (or datetime)",
    title; "Defines a title for the document",
    tr; "Defines a row in a table",
    track; "Defines text tracks for media elements (<video> and <audio>)",
    u; "Defines some text that is unarticulated and styled differently from normal text",
    ul; "Defines an unordered list",
    var; "Defines a variable",
    video; "Defines embedded video content",
    wbr; "Defines a possible line-break"
);

/// Creates a `main` html element.
/// Specifies the main content of a document
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

impl HtmlElement {
    set_attr!(
        accesskey,
        alt,
        class,
        contenteditable,
        decoding,
        data_tip = "data-tip",
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
    use crate::render::Render;

    #[test]
    fn children_macro() {
        let res = div()
            .add_children(children!["Some text", p().add_child("inner text"), 123456])
            .render();
        insta::assert_snapshot!(res, @r"
        <div>
          Some text
          <p>inner text</p>
          123456
        </div>
        ");
    }

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
}
