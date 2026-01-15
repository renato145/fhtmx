use crate::macros::{daisy_class, daisy_component};
use fhtmx::prelude::*;

daisy_component!(
    Button = button().class("btn");
    "Buttons allow the user to take actions or make choices."
);

impl Button {
    daisy_class!(
        // Colors
        neutral = "btn-neutral"; "neutral color",
        primary = "btn-primary"; "primary color",
        secondary = "btn-secondary"; "secondary color",
        accent = "btn-accent"; "accent color",
        info = "btn-info"; "info color",
        success = "btn-success"; "success color",
        warning = "btn-warning"; "warning color",
        error = "btn-error"; "error color",

        // Styles
        outline = "btn-outline"; "outline style",
        dash = "btn-dash"; "dash style",
        soft = "btn-soft"; "soft style",
        ghost = "btn-ghost"; "ghost style",
        link = "btn-link"; "looks like a link",

        // Behavior
        active = "btn-active"; "looks active",
        disabled = "btn-disabled"; "looks disabled",

        // Sizes
        xs = "btn-xs"; "Extra small size",
        sm = "btn-sm"; "Small size",
        md = "btn-md"; "Medium size (default)",
        lg = "btn-lg"; "Large size",
        xl = "btn-xl"; "Extra large size",

        // Modifiers
        wide = "btn-wide"; "more horizontal padding",
        block = "btn-block"; "Full width",
        square = "btn-square"; "1:1 ratio",
        circle = "btn-circle"; "1:1 ratio with rounded corners",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_works() {
        let res = dc_button().primary().xs().html().add_class("px-2").render();
        insta::assert_snapshot!(res, @r#"<button class="btn btn-primary btn-xs px-2"></button>"#);
    }
}
