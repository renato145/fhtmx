use crate::{
    attribute::{AttributeValue, IntoAttributeValue},
    element::{Element, set_attr, set_empty_attr},
    node::{HtmlNode, IntoNode},
};
use indexmap::{IndexMap, IndexSet};
use pastey::paste;
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
    pub classes: IndexSet<Cow<'static, str>>,
    pub children: Vec<HtmlNode>,
}

// TODO: implement commented methods
impl HtmlElement {
    pub fn new(tag: &'static str) -> Self {
        Self {
            tag,
            attrs: IndexMap::new(),
            classes: IndexSet::new(),
            children: Vec::new(),
        }
    }
}

pub trait IntoHtmlElement {
    /// Transforms into a `HtmlElement`
    fn into_element(self) -> HtmlElement;
}

impl IntoHtmlElement for HtmlElement {
    #[inline]
    fn into_element(self) -> HtmlElement {
        self
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
    fn attrs_mut(&mut self) -> &mut IndexMap<Cow<'static, str>, AttributeValue> {
        &mut self.attrs
    }

    #[inline]
    fn classes(&self) -> &IndexSet<Cow<'static, str>> {
        &self.classes
    }

    #[inline]
    fn classes_mut(&mut self) -> &mut IndexSet<Cow<'static, str>> {
        &mut self.classes
    }

    #[inline]
    fn children(&self) -> &[HtmlNode] {
        &self.children
    }

    #[inline]
    fn children_mut(&mut self) -> &mut Vec<HtmlNode> {
        &mut self.children
    }

    #[inline]
    fn is_void_tag(&self) -> bool {
        VOID_ELEMENTS.contains(&self.tag())
    }

    #[inline]
    fn is_inline_tag(&self) -> bool {
        INLINE_ELEMENTS.contains(&self.tag())
    }
}

impl<T: IntoHtmlElement> IntoNode for T {
    fn into_node(self) -> HtmlNode {
        HtmlNode::Element(self.into_element())
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

    ($name:ident$(;$eg:expr)?, $($rest:ident$(;$eg_rest:expr)?),+ $(,)?) => {
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
    wbr; "Defines a possible line-break",
);

/// Creates a `main` html element.
/// Specifies the main content of a document
pub fn main_tag() -> HtmlElement {
    HtmlElement::new("main")
}

impl HtmlElement {
    set_attr!(
        accesskey,
        alt,
        contenteditable,
        decoding,
        data_tip = "data-tip",
        dir,
        draggable,
        enterkeyhint,
        for_ = "for",
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
        role,
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
        autofocus, blocking, checked, defer, disabled, hidden, inert, multiple, nomodule, open,
        popover, r#async, readonly, required, selected
    );
}
