use fhtmx::{elements::*, html_page::HtmlPage};

fn labelled_fieldset(label: &str) -> HtmlElement<&str, HtmlGenericElement> {
    fieldset()
        .class("fieldset w-xs bg-base-300 p-4 rounded-box")
        .add_child(legend().class("fieldset-legend").inner(label))
}

fn input_field(input_label: &str, typ: &str, placeholder: &str) -> HtmlElements {
    vec![
        label().class("fieldset-label").inner(input_label).boxed(),
        input()
            .class("input")
            .r#type(typ)
            .placeholder(placeholder)
            .boxed(),
    ]
}

fn main() {
    let body = main_tag()
        .class("container mx-auto mt-4")
        .add_child(
            h1().inner("A Form example with tailwind")
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
                        .add_child(button().class("btn btn-primary mt-4").inner("Register")),
                )
                .add_child(
                    labelled_fieldset("Professional data")
                        .add_children(input_field("Profession", "text", "Your profession"))
                        .add_children(input_field("Experience", "number", "Number of years"))
                        .add_children(vec![
                            label()
                                .class("fieldset-label")
                                .inner("Currently working?")
                                .boxed(),
                            select()
                                .class("select")
                                .add_child(option().inner("Yes"))
                                .add_child(option().inner("No"))
                                .boxed(),
                        ])
                        .add_child(button().class("btn btn-primary mt-4").inner("Register")),
                ),
        );
    let page = HtmlPage::new()
        .custom_html_node(html().set_attr("data-theme", "dark").set_attr("lang", "en"))
        .title("daisyui form")
        .add_header_child(
            link()
                .href("https://cdn.jsdelivr.net/npm/daisyui@5")
                .rel("stylesheet")
                .r#type("text/css"),
        )
        .add_header_child(script().src("https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"))
        .add_body_child(body)
        .render();
    std::fs::write("examples_outputs/daisyui_form.html", page).unwrap();
    println!("Done!");
}
