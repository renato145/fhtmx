use crate::{
    element::Element,
    html_element::*,
    node::IntoNode,
    prelude::{dc_fieldset, dc_fieldset_legend},
    svg::SvgElement,
};

/// Styled fieldset to label a form input
/// Container for form inputs, inside use flatten labels and inputs.
pub fn mk_fieldset_container(lbl: impl IntoNode) -> HtmlElement {
    dc_fieldset().add(dc_fieldset_legend().add(lbl))
}

/// Styled label for a form input (you don't need to add "input" class to the `input`` form field)
pub fn mk_labelled_input(lbl: impl IntoNode, input: impl IntoNode) -> HtmlElement {
    label()
        .class("input")
        .add(span().class("label").add(lbl))
        .add(input)
}

/// Styled label for a form input (you don't need to add "input" class to the input form field)
pub fn mk_labelled_input_with_icon(icon: SvgElement, input: impl IntoNode) -> HtmlElement {
    label()
        .class("input")
        .add(icon.add_class("h-[1em] opacity-50"))
        .add(input)
}

/// Disabled option, use it as a placeholder
pub fn option_disabled(lbl: &str) -> HtmlElement {
    option().disabled().selected().value("").add(lbl)
}
