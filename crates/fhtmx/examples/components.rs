use fhtmx::prelude::*;
use lipsum::lipsum;

fn wrapper(title: &str, node: impl IntoNode) -> HtmlElement {
    mk_centered_container()
        .add_class("mt-4 p-4 rounded-box items-start")
        .add_class(DaisyColor::Base300.bg_content())
        .add(h2().class("text-2xl font-semibold mb-4").add(title))
        .add(node)
}

fn icons_group(title: &str, icons: Vec<(SvgElement, &str)>) -> HtmlElement {
    mk_card(
        Some(title),
        div()
            .class("flex flex-wrap gap-4")
            .add_children(icons.into_iter().map(|(icon, name)| {
                div()
                    .class("flex flex-col items-center")
                    .add(icon.class("h-12 w-12"))
                    .add(name)
            })),
    )
}

fn main() {
    let general_icons = vec![
        (icons::menu(), "menu"),
        (icons::user(), "user"),
        (icons::search(), "search"),
        (icons::email(), "email"),
        (icons::password(), "password"),
        (icons::pin(), "pin"),
        (icons::refresh(), "refresh"),
        (icons::sun(), "sun"),
        (icons::moon(), "moon"),
    ];
    let form_icons = vec![
        (icons::edit(), "edit"),
        (icons::save(), "save"),
        (icons::delete(), "delete"),
    ];
    let callout_icons = vec![
        (icons::info(), "info"),
        (icons::note(), "note"),
        (icons::important(), "important"),
        (icons::tip(), "tip"),
        (icons::caution(), "caution"),
        (icons::warning(), "warning"),
        (icons::error(), "error"),
        (icons::success(), "success"),
    ];
    let icons = fragment([
        icons_group("General", general_icons),
        icons_group("Form", form_icons),
        icons_group("Callout", callout_icons),
    ]);

    let alerts = div().class("flex flex-col gap-y-2").add_children([
        mk_alert_info(&lipsum(15)),
        mk_alert_success(&lipsum(15)),
        mk_alert_warning(&lipsum(15)),
        mk_alert_error(&lipsum(15)),
    ]);

    let body = main_container()
        .add_class("mt-4")
        .add(
            h1().add("Components using DaisyUI")
                .class("text-2xl font-bold text-center"),
        )
        .add(wrapper("Icons", icons))
        .add(wrapper("Alerts", alerts));

    let page = HtmlPage::new()
        .custom_html_node(html().set_attr("data-theme", "dark").lang("en"))
        .title("Components")
        .add_header_node(daisy_link())
        .add_header_node(source_tailwind())
        .add_body_node(body)
        .render();
    std::fs::write("examples/components.html", page).unwrap();
    println!("Done!");
}
