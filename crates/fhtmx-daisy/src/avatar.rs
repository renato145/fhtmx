use crate::macros::{daisy_class, daisy_component};
use fhtmx::prelude::*;

daisy_component!(
    Avatar = div().class("avatar");
    "Avatars are used to show a thumbnail representation of an individual or business in the interface."
);

impl Avatar {
    daisy_class!(
        // Presence indicators
        online = "avatar-online"; "Shows a green dot as online indicator",
        offline = "avatar-offline"; "Shows a gray dot as offline indicator",

        // Modifier
        placeholder = "avatar-placeholder"; "To show letters as avatar placeholder",
    );
}

daisy_component!(
    AvatarGroup = div().class("avatar-group");
    "Container for multiple avatars"
);
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_works() {
        let res =
            dc_avatar()
                .html()
                .add(div().class("w-24 rounded").add(
                    img().src("https://img.daisyui.com/images/profile/demo/batperson@192.webp"),
                ))
                .render();
        insta::assert_snapshot!(res, @r#"
        <div class="avatar">
          <div class="w-24 rounded"><img src="https://img.daisyui.com/images/profile/demo/batperson@192.webp" /></div>
        </div>
        "#);
    }
}
