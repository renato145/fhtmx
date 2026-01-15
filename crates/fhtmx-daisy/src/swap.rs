use crate::macros::{daisy_class, daisy_component};
use fhtmx::prelude::*;

daisy_component!(
    Swap = label().class("swap");
    "Swap allows you to toggle the visibility of two elements using a checkbox or a class name."
);

impl Swap {
    daisy_class!(
        // Styles
        rotate = "swap-rotate"; "Adds rotate effect to swap",
        flip = "swap-flip"; "Adds flip effect to swap",

        // Modifier
        active = "swap-active"; "Activates the swap (no need for checkbox)",
    );
}

daisy_component!(
    SwapOn = div().class("swap-on");
    "The child element that should be visible when checkbox is checked or when swap is active"
);

daisy_component!(
    SwapOff = div().class("swap-off");
    "The child element that should be visible when checkbox is not checked or when swap is not active"
);

daisy_component!(
    SwapIndeterminate = div().class("swap-indeterminate");
    "The child element that should be visible when checkbox is indeterminate"
);

// TODO: assemble utility

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_works() {
        let res = dc_swap()
            .html()
            .add(dc_swap_on().html().add("ON"))
            .add(dc_swap_off().html().add("OFF"))
            .render();

        insta::assert_snapshot!(res, @r#"
        <label class="swap">
          <div class="swap-on">ON</div>
          <div class="swap-off">OFF</div>
        </label>
        "#);
    }
}
