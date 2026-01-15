use crate::macros::{daisy_class, daisy_component};
use fhtmx::prelude::*;

daisy_component!(
    Accordion = details().class("collapse");
    "Accordion is used for showing and hiding content but only one item can stay open at a time."
);

impl Accordion {
    daisy_class!(
        // Icons
        arrow = "collapse-arrow"; "Adds arrow icon",
        plus = "collapse-plus"; "Adds plus/minus icon",

        // Modifiers
        open = "collapse-open"; "Force open",
        close = "collapse-close"; "Force close",
    );
}

daisy_component!(
    AccordionTitle = summary().class("collapse-title");
    "Title part of accordion"
);

daisy_component!(
    AccordionContent = div().class("collapse-content");
    "Content part of accordion"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_works() {
        let res = fragment([
            dc_accordion()
                .html()
                .add_class("bg-base-100 border border-base-300")
                .name("my-accordion")
                .open()
                .add(
                    dc_accordion_title()
                        .html()
                        .add_class("font-semibold")
                        .add("How do I create an account?"),
                )
                .add(
                    dc_accordion_content()
                        .html()
                        .add_class("text-sm")
                        .add("Content 1"),
                ),
            dc_accordion()
                .html()
                .add_class("bg-base-100 border border-base-300")
                .name("my-accordion")
                .add(
                    dc_accordion_title()
                        .html()
                        .add_class("font-semibold")
                        .add("I forgot my password. What should I do?"),
                )
                .add(
                    dc_accordion_content()
                        .html()
                        .add_class("text-sm")
                        .add("Content 2"),
                ),
            dc_accordion()
                .html()
                .add_class("bg-base-100 border border-base-300")
                .name("my-accordion")
                .add(
                    dc_accordion_title()
                        .html()
                        .add_class("font-semibold")
                        .add("How do I update my profile information?"),
                )
                .add(
                    dc_accordion_content()
                        .html()
                        .add_class("text-sm")
                        .add("Content 3"),
                ),
        ])
        .render();
        insta::assert_snapshot!(res, @r#"
        <details class="collapse bg-base-100 border border-base-300" name="my-accordion" open>
          <summary class="collapse-title font-semibold">How do I create an account?</summary>
          <div class="collapse-content text-sm">Content 1</div>
        </details>
        <details class="collapse bg-base-100 border border-base-300" name="my-accordion">
          <summary class="collapse-title font-semibold">I forgot my password. What should I do?</summary>
          <div class="collapse-content text-sm">Content 2</div>
        </details>
        <details class="collapse bg-base-100 border border-base-300" name="my-accordion">
          <summary class="collapse-title font-semibold">How do I update my profile information?</summary>
          <div class="collapse-content text-sm">Content 3</div>
        </details>
        "#);
    }
}
