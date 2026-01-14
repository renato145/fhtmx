use crate::macros::{daisy_class, daisy_component};
use fhtmx::prelude::*;

daisy_component!(Button = button(); "btn");
daisy_class!(
    Button(
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
        link = "btn-link"; "link style",
        // State
        active = "btn-active"; "active state",
        disabled = "btn-disabled"; "disabled state",
        // Sizes
        xs = "btn-xs"; "extra small size",
        sm = "btn-sm"; "small size",
        md = "btn-md"; "medium size",
        lg = "btn-lg"; "large size",
        xl = "btn-xl"; "extra large size",
        // Modifiers
        wide = "btn-wide"; "wide button",
        block = "btn-block"; "full width button",
        square = "btn-square"; "square button",
        circle = "btn-circle"; "circle button",
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_works() {
        let res = ds_button().primary().xs().add_class("px-2").render();
        insta::assert_snapshot!(res, @r#"<button class="btn btn-primary btn-xs px-2"></button>"#);
    }
}
