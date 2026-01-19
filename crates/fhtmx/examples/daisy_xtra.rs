use fhtmx::prelude::*;
use lipsum::lipsum;

fn wrapper(title: &str, node: impl IntoNode) -> HtmlElement {
    mk_centered_container(Some(DaisyColor::Base300))
        .add_class("mt-4 p-4 rounded-box items-start")
        .add(h2().class("text-lg font-semibold mb-4").add(title))
        .add(node)
}

fn main() {
    let cards = div().class("w-full grid grid-cols-3 gap-4").add_children(
        [
            ("Xsmall Card", "card-xs"),
            ("Small Card", "card-sm"),
            ("Medium Card (default)", ""),
            ("Large Card", "card-lg"),
            ("Xlarge Card", "card-xl"),
        ]
        .map(|(title, cls)| {
            mk_card(Some(title), lipsum(10))
                .add_class(cls)
                .add_class(DaisyColor::Info.bg_content())
        }),
    );

    let dropdown = mk_dropdown(
        "open or close",
        ["Item 1", "Item 2"].map(|o| li().add(a().add(o))),
        "btn m-1",
        "bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm",
    );

    let fab = div().class("relative w-full h-28").add(
        mk_fab(
            "F",
            ["A", "B", "C"].map(|o| dc_btn().add_class("btn-lg btn-circle").add(o)),
            "btn-lg btn-circle btn-primary",
        )
        .add_class("absolute"),
    );

    let swap = mk_swap("ON", "OFF");

    let accordion = mk_accordion(
        [
            (
                "How do I create an account?",
                r#"Click the "Sign Up" button in the top right corner and follow the registration process"#,
            ),
            (
                "I forgot my password. What should I do?",
                r#"Click on "Forgot Password" on the login page and follow the instructions sent to your email."#,
            ),
            (
                "How do I update my profile information?",
                r#"Go to "My Account" settings and select "Edit Profile" to make changes."#,
            ),
        ],
        "bg-base-100 border border-base-300",
        "font-semibold",
        "text-sm",
        Some("my-accordion".to_string()),
    );

    let body = main_container()
        .add_class("mt-4")
        .add(
            h1().add("Components using DaisyUI")
                .class("text-2xl font-bold text-center"),
        )
        .add(wrapper("Cards", cards))
        .add(wrapper("Dropdown", dropdown))
        .add(wrapper("FAB (Floating Action Button)", fab))
        .add(wrapper("Swap", swap))
        .add(wrapper("Accordion", accordion));

    let page = HtmlPage::new()
        .custom_html_node(html().set_attr("data-theme", "dark").lang("en"))
        .title("daisyui xtra")
        .add_header_node(daisy_link())
        .add_header_node(source_tailwind())
        .add_body_node(body)
        .render();
    std::fs::write("examples/daisy_xtra.html", page).unwrap();
    println!("Done!");
}
