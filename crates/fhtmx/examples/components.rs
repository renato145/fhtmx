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

    let alerts = div()
        .class("w-full grid grid-cols-1 md:grid-cols-2 gap-2")
        .add_children([
            mk_alert_info(format!("Info alert: {}", lipsum(15))),
            mk_alert_success(format!("Sucess alert: {}", lipsum(15))),
            mk_alert_warning(format!("Warning alert: {}", lipsum(15))),
            mk_alert_error(format!("Error alert: {}", lipsum(15))),
        ]);

    let callout_blocks = div()
        .class("w-full grid grid-cols-1 md:grid-cols-2 gap-4")
        .add(mk_callout_note(
            None,
            p().add("Note that there are five types of callout_blocks")
                .add(span().class("font-bold").add("callout_blocks"))
                .add(", including: ")
                .add(
                    span()
                        .class("font-bold")
                        .add("note, tip, warning, caution and important."),
                ),
            false,
        ))
        .add(mk_callout_warning(
            None,
            "Callouts provide a simple way to attract attention, for example, to this warning.",
            false,
        ))
        .add(mk_callout_error(None, "Some error message here...", false))
        .add(mk_callout_important(
            None,
            "The callout heading is provided by the callout type, with the expected heading \
             (i.e., Note, Warning, Important, Tip, or Caution).",
            false,
        ))
        .add(mk_callout_tip(
            Some("with some title"),
            "This is an example of a callout with a title. \
             Providing a callout title is optional.",
            false,
        ))
        .add(mk_callout_caution(
            Some("with collapse"),
            "This is an example of a ‘collapsed’ caution callout that can be expanded by the user.",
            true,
        ));

    let toasts = div()
        .class("w-full")
        .set_attr("x-data", "{ items: [], i: 0 }")
        .add(
            dc_btn()
                .add_class("btn-primary")
                .set_attr("@click", "items.push(i++)")
                .add("Spawn toast"),
        )
        .add(
            div().class("toast").add(
                template()
                    .set_attr("x-for", "item in items")
                    .add(mk_alert_success("Some success message :)").setup_toast(false)),
            ),
        );

    let lazy_load = div()
        .class("w-full flex flex-col gap-4")
        .add(lazy_load_bars_xs(Some("extra small")))
        .add(lazy_load_bars_sm(Some("small")))
        .add(lazy_load(Some("medium (default)")))
        .add(lazy_load_bars_lg(Some("large")))
        .add(lazy_load_bars_xl(Some("extra large")))
        .add(
            div()
                .class("flex flex-wrap gap-4 items-center")
                .add(p().class("mt-2 text-lg font-semibold").add("Other icons:"))
                .add(lazy_load_spinner(None))
                .add(lazy_load_dots(None))
                .add(lazy_load_ring(None))
                .add(lazy_load_ball(None))
                .add(lazy_load_infinity(None)),
        );

    let body = main_container()
        .add_class("mt-4")
        .add(
            h1().add("Components using DaisyUI")
                .class("text-2xl font-bold text-center"),
        )
        .add(wrapper("Theme change", theme_toogle_with_size(10)))
        .add(wrapper("Icons", icons))
        .add(wrapper("Alerts", alerts))
        .add(wrapper("Callout blocks", callout_blocks))
        .add(wrapper("Toasts", toasts))
        .add(wrapper("Lazy load", lazy_load));

    let page = HtmlPage::new()
        .custom_html_node(html().set_attr("data-theme", "dark").lang("en"))
        .title("Components")
        .add_header_node(daisy_link())
        .add_header_node(source_tailwind())
        .add_header_node(source_alpinejs_persist())
        .add_header_node(source_alpinejs())
        .add_header_node(script_setup_toast())
        .add_header_node(script_setup_theme("light", "dark"))
        .add_body_node(body)
        .render();
    std::fs::write("examples/components.html", page).unwrap();
    println!("Done!");
}
