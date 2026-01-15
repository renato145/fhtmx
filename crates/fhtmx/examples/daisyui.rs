use fhtmx::prelude::*;

fn labelled_fieldset(label: &str) -> HtmlElement {
    fieldset()
        .class("fieldset w-xs bg-base-300 p-4 rounded-box")
        .add(legend().class("fieldset-legend").add(label))
}

fn input_field(input_label: &str, typ: &str, placeholder: &str) -> Vec<HtmlNode> {
    children![
        label().class("fieldset-label").add(input_label),
        input().class("input").typ(typ).placeholder(placeholder),
    ]
}

fn main() {
    let body = main_tag()
        .class("container mx-auto mt-4")
        .add(
            h1().add("A Form example with tailwind")
                .class("text-2xl font-bold text-center"),
        )
        .add(
            div()
                .class("mt-4 p-4 flex flex-wrap justify-center gap-8 bg-base-100 rounded-box")
                .add(
                    labelled_fieldset("Personal data")
                        .add_children(input_field("Name", "text", "Your name here"))
                        .add_children(input_field("Last name", "text", "Your last name here"))
                        .add_children(input_field("Name", "text", "Your name here"))
                        .add(button().class("btn btn-primary mt-4").add("Register")),
                )
                .add(
                    labelled_fieldset("Professional data")
                        .add_children(input_field("Profession", "text", "Your profession"))
                        .add_children(input_field("Experience", "number", "Number of years"))
                        .add_children(children![
                            label().class("fieldset-label").add("Currently working?"),
                            select()
                                .class("select")
                                .add(option().add("Yes"))
                                .add(option().add("No")),
                        ])
                        .add(button().class("btn btn-primary mt-4").add("Register")),
                ),
        );
    let page = HtmlPage::new()
        .custom_html_node(html().set_attr("data-theme", "dark").lang("en"))
        .title("daisyui form")
        .add_header_node(
            link()
                .href("https://cdn.jsdelivr.net/npm/daisyui@5")
                .rel("stylesheet")
                .typ("text/css"),
        )
        .add_header_node(source_tailwind())
        .add_body_node(body)
        .render();
    std::fs::write("examples/daisyui.html", page).unwrap();
    println!("Done!");
}
