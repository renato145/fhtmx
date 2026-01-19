use fhtmx::prelude::*;

fn labelled_fieldset(label: &str, inputs: &[(&str, &str, &str)]) -> HtmlElement {
    dc_fieldset()
        .add_class("w-xs bg-base-300 p-4 rounded-box")
        .add(dc_fieldset_legend().add(label))
        .add_children(inputs.iter().flat_map(|&(lbl, typ, placeholder)| {
            [
                dc_label().add(lbl),
                dc_input().typ(typ).placeholder(placeholder),
            ]
        }))
}

fn main() {
    let body = main_container()
        .add_class("mt-4")
        .add(
            h1().add("A Form example with DaisyUI")
                .class("text-2xl font-bold text-center"),
        )
        .add(
            div()
                .class("mt-4 p-4 flex flex-wrap justify-center gap-8 bg-base-100 rounded-box")
                .add(
                    labelled_fieldset(
                        "Personal data",
                        &[
                            ("Name", "text", "Your name here"),
                            ("Last name", "text", "Your last name here"),
                            ("Name", "text", "Your name here"),
                        ],
                    )
                    .add(dc_btn().add_class("btn-primary mt-4").add("Register")),
                )
                .add(
                    labelled_fieldset(
                        "Professional data",
                        &[
                            ("Profession", "text", "Your profession"),
                            ("Experience", "number", "Number of years"),
                        ],
                    )
                    .add_children(children![
                        dc_label().add("Currently working?"),
                        dc_select().add(option().add("Yes")).add(option().add("No")),
                    ])
                    .add(dc_btn().add_class("btn-primary mt-4").add("Register")),
                ),
        );
    let page = HtmlPage::new()
        .custom_html_node(html().set_attr("data-theme", "dark").lang("en"))
        .title("daisyui form")
        .add_header_node(daisy_link())
        .add_header_node(source_tailwind())
        .add_body_node(body)
        .render();
    std::fs::write("examples/daisy_form.html", page).unwrap();
    println!("Done!");
}
