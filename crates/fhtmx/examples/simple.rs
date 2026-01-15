use fhtmx::prelude::*;

fn main() {
    let squares = (0..10)
        .map(|i| {
            div()
                .class(
                    "h-24 flex items-center justify-center rounded-md bg-linear-to-r from-emerald-400 to-emerald-700 border-2 border-black",
                )
                .add(p().add(format!("Box: {i}")).class("text-lg font-bold text-shadow-lg/30"))
        })
        .collect::<Vec<_>>();
    let body = main_tag()
        .class("container mx-auto mt-4")
        .add(
            h1().add("Example with tailwind")
                .class("text-2xl font-bold text-center"),
        )
        .add(
            div()
                .class("mt-4 p-4 grid grid-cols-4 gap-4")
                .add_children(squares),
        );
    let page = HtmlPage::new()
        .title("simple")
        .add_header_node(source_tailwind())
        .add_header_node(
            style()
                .typ("text/tailwindcss")
                .add("body { @apply bg-sky-950 text-slate-100; }"),
        )
        .add_body_node(body)
        .render();
    std::fs::write("examples/simple.html", page).unwrap();
    println!("Done!");
}
