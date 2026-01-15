use crate::macros::{daisy_class, daisy_component};
use fhtmx::prelude::*;

daisy_component!(
    Dropdown = details().class("dropdown");
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

daisy_component!(
    DropdownContent = ul().class("dropdown-content");
    "Content part of a dropdown."
);

// TODO: assemble utility

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropdown_works() {
        let res = dc_dropdown()
            .html()
            .add(summary().class("btn m-1").add("open or close"))
            .add(
                dc_dropdown_content()
                    .html()
                    .add_class("menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm")
                    .add(li().add(a().add("Item 1")))
                    .add(li().add(a().add("Item 2"))),
            )
            .render();
        insta::assert_snapshot!(res, @r#"
        <details class="dropdown">
          <summary class="btn m-1">open or close</summary>
          <ul class="dropdown-content menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm">
            <li><a>Item 1</a></li>
            <li><a>Item 2</a></li>
          </ul>
        </details>
        "#);
    }
}
