use fhtmx::prelude::*;

fn main() {
    let tmp = mk_card(
        Some("Some title"),
        vec![
            p().add("Some content."),
            dc_card_actions()
                .add(dc_btn().add_class("btn-success btn-sm").add("Ok"))
                .add(dc_btn().add_class("btn-error btn-sm").add("Cancel")),
        ],
    )
    .add_class(DaisyColor::Base300.bg_content());

    let body = main_container()
        .add_class("mt-4")
        .add(
            h1().add("Html view example")
                .class("text-2xl font-bold text-center"),
        )
        .add(
            div()
                .class("mt-4 p-2 flex flex-wrap gap-4")
                .add(tmp.clone())
                .add(tmp.clone())
                .add(tmp.clone())
                .add(tmp),
        );
    let page = HtmlPage::new()
        .custom_html_node(html().set_attr("data-theme", "dark").lang("en"))
        .title("Html view")
        .add_header_node(daisy_link())
        .add_header_node(source_tailwind())
        .add_body_node(body)
        .render();
    std::fs::write("examples/html_view.html", page).unwrap();
    println!("Done!");
}
