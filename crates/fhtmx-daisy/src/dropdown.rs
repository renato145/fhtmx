use crate::macros::{daisy_class, daisy_component};
use fhtmx::prelude::*;

daisy_component!(
    Dropdown = details(); "dropdown";
    "Dropdown can open a menu or any other element when the button is clicked."
);

impl Dropdown {
    daisy_class!(
        // Placement - horizontal alignment
        start = "dropdown-start"; "Align horizontally to start of button",
        center = "dropdown-center"; "Align horizontally to center of button",
        end = "dropdown-end"; "Align horizontally to end of button",

        // Placement - direction
        top = "dropdown-top"; "Open from top",
        bottom = "dropdown-bottom"; "Open from bottom (default)",
        left = "dropdown-left"; "Open from left",
        right = "dropdown-right"; "Open from right",

        // Modifiers
        hover = "dropdown-hover"; "Opens on hover too",
        open = "dropdown-open"; "Force open",
        close = "dropdown-close"; "Force close",
    );
}

daisy_component!(DropdownContent = ul(); "dropdown-content"; "Content part of a dropdown.");

// TODO: assemble utility

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropdown_works() {
        let res = ds_dropdown()
            .left()
            .html()
            .add(ds_dropdown_content())
            .render();
        insta::assert_snapshot!(res, @r#"
        <details class="dropdown dropdown-left">
          <ul class="dropdown-content"></ul>
        </details>
        "#);
    }
}
