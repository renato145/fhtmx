use crate::macros::{daisy_class, daisy_component};
use fhtmx::prelude::*;

daisy_component!(Dropdown = details(); "dropdown");
daisy_class!(
    Dropdown(
        // Position
        dropdown_end = "dropdown-end"; "align to end",
        dropdown_top = "dropdown-top"; "open to top",
        dropdown_bottom = "dropdown-bottom"; "open to bottom",
        dropdown_left = "dropdown-left"; "open to left",
        dropdown_right = "dropdown-right"; "open to right",
        // Behavior
        dropdown_hover = "dropdown-hover"; "open on hover",
        dropdown_open = "dropdown-open"; "force open state",
    )
);

daisy_component!(DropdownContent = ul(); "dropdown-content");

// TODO: assemble utility

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropdown_works() {
        let res = ds_dropdown()
            .dropdown_left()
            .html()
            .add_child(ds_dropdown_content())
            .render();
        insta::assert_snapshot!(res, @r#"
        <details class="dropdown dropdown-left">
          <ul class="dropdown-content"></ul>
        </details>
        "#);
    }
}
