use fhtmx::prelude::*;

fn labelled_fieldset(label: &str) -> HtmlElement {
    fieldset()
        .class("fieldset w-xs bg-base-300 p-4 rounded-box")
        .add_child(legend().class("fieldset-legend").add_child(label))
}

fn input_field(input_label: &str, typ: &str, placeholder: &str) -> Vec<HtmlNode> {
    children![
        label().class("fieldset-label").add_child(input_label),
        input().class("input").typ(typ).placeholder(placeholder),
    ]
}

fn main() {
    let body = main_tag()
        .class("container mx-auto mt-4")
        .add_child(
            h1().add_child("A Form example with tailwind")
                .class("text-2xl font-bold text-center"),
        )
        .add_child(
            div()
                .class("mt-4 p-4 flex flex-wrap justify-center gap-8 bg-base-100 rounded-box")
                .add_child(
                    labelled_fieldset("Personal data")
                        .add_children(input_field("Name", "text", "Your name here"))
                        .add_children(input_field("Last name", "text", "Your last name here"))
                        .add_children(input_field("Name", "text", "Your name here"))
                        .add_child(button().class("btn btn-primary mt-4").add_child("Register")),
                )
                .add_child(
                    labelled_fieldset("Professional data")
                        .add_children(input_field("Profession", "text", "Your profession"))
                        .add_children(input_field("Experience", "number", "Number of years"))
                        .add_children(children![
                            label()
                                .class("fieldset-label")
                                .add_child("Currently working?"),
                            select()
                                .class("select")
                                .add_child(option().add_child("Yes"))
                                .add_child(option().add_child("No")),
                        ])
                        .add_child(button().class("btn btn-primary mt-4").add_child("Register")),
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
        .add_header_node(script().src("https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"))
        .add_body_node(body)
        .render();
    std::fs::write("examples/daisyui.html", page).unwrap();
    println!("Done!");
}
