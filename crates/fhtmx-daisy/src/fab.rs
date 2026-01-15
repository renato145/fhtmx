use crate::macros::{daisy_class, daisy_component};
use fhtmx::prelude::*;

daisy_component!(
    Fab = div(); "fab";
    "FAB (Floating Action Button) stays in the bottom corner of screen. Clicking or focusing it shows additional buttons."
);

impl Fab {
    daisy_class!(
        // Modifier
        flower = "fab-flower"; "Open in a quarter circle arrangement instead of vertical",
    );
}

daisy_component!(
    FabClose = div(); "fab-close";
    "Optional close button wrapper that replaces the main button when FAB is open."
);

daisy_component!(
    FabMainAction = div(); "fab-main-action";
    "Optional main action button wrapper that replaces the main button when FAB is open."
);

// TODO: assemble utility

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fab_works() {
        let res = ds_fab()
            .html()
            .add(
                div()
                    .tabindex(0)
                    .role("button")
                    .class("btn btn-lg btn-circle btn-success"),
            )
            .add(ds_fab_close().html().add("Close"))
            .add(button().class("btn btn-lg btn-circle").add("A"))
            .add(button().class("btn btn-lg btn-circle").add("B"))
            .render();
        insta::assert_snapshot!(res, @r#"
        <div class="fab">
          <div tabindex="0" role="button" class="btn btn-lg btn-circle btn-success"></div>
          <div class="fab-close">Close</div>
          <button class="btn btn-lg btn-circle">A</button>
          <button class="btn btn-lg btn-circle">B</button>
        </div>
        "#);
    }
}
