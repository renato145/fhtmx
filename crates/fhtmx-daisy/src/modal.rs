use crate::macros::{daisy_class, daisy_component};
use fhtmx::prelude::*;

daisy_component!(
    ModalToggle = input().class("modal-toggle").typ("checkbox");
    "Hidden checkbox that controls the state of modal"
);

daisy_component!(
    Modal = div().class("modal").role("dialog");
    "Modal is used to show a dialog or a box when you click a button."
);

impl Modal {
    daisy_class!(
        // Modifier
        open = "modal-open"; "Keeps the modal open",

        // Placement - vertical
        top = "modal-top"; "Moves the modal to top",
        middle = "modal-middle"; "Moves the modal to middle (default)",
        bottom = "modal-bottom"; "Moves the modal to bottom",

        // Placement - horizontal
        start = "modal-start"; "Moves the modal to start horizontally",
        end = "modal-end"; "Moves the modal to end horizontally",
    );
}

daisy_component!(
    ModalBox = div().class("modal-box");
    "The content part of modal"
);

daisy_component!(
    ModalAction = div().class("modal-action");
    "Actions part (buttons, etc.)"
);

daisy_component!(
    ModalBackdrop = label().class("modal-backdrop");
    "Label that covers the page when modal is open so we can close the modal by clicking outside"
);

// TODO: assemble utility

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fab_works() {
        let res = fragment([
            label().for_("my_modal").class("btn").add("open modal"),
            dc_modal_toggle().html().id("my_modal"),
            dc_modal().html().add(
                dc_modal_box()
                    .html()
                    .add(h3().class("text-lg font-bold").add("Hello!"))
                    .add(
                        p().class("py-4")
                            .add("This modal works with a hidden checkbox!"),
                    )
                    .add(
                        dc_modal_action()
                            .html()
                            .add(label().for_("my_modal").class("btn").add("Close!")),
                    ),
            ),
        ])
        .render();
        insta::assert_snapshot!(res, @r#"
        <label class="btn" for="my_modal">open modal</label>
        <input class="modal-toggle" type="checkbox" id="my_modal" />
        <div class="modal" role="dialog">
          <div class="modal-box">
            <h3 class="text-lg font-bold">Hello!</h3>
            <p class="py-4">This modal works with a hidden checkbox!</p>
            <div class="modal-action"><label class="btn" for="my_modal">Close!</label></div>
          </div>
        </div>
        "#);
    }
}
