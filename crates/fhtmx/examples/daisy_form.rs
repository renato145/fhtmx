use fhtmx::prelude::*;

fn labelled_fieldset(lbl: &str, inputs: &[(&str, &str, &str)]) -> HtmlElement {
    mk_fieldset_container(lbl)
        .add_class("w-xs bg-base-300 p-4 rounded-box")
        .add_children(inputs.iter().flat_map(|&(lbl, typ, placeholder)| {
            [
                dc_label().add(lbl),
                dc_input().typ(typ).placeholder(placeholder),
            ]
        }))
}

fn main() {
    let body = main_container()
        .add(
            h1().add("A Form example with DaisyUI")
                .class("py-6 text-2xl font-bold text-center"),
        )
        .add(
            div()
                .class("mt-2 flex flex-wrap justify-center gap-8")
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
