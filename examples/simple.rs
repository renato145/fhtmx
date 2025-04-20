use fhtmx::prelude::*;

fn main() {
    let squares = (0..10)
        .map(|i| {
            div()
                .class(
                    "h-24 flex items-center justify-center rounded-md bg-linear-to-r from-emerald-400 to-emerald-700 border-2 border-black",
                )
                .add_child(p().inner(&format!("Box: {}", i)).class("text-lg font-bold text-shadow-lg/30"))
                .boxed()
        })
        .collect::<Vec<_>>();
    let body = main_tag()
        .class("container mx-auto mt-4")
        .add_child(
            h1().inner("Example with tailwind")
                .class("text-2xl font-bold text-center"),
        )
        .add_child(
            div()
                .class("mt-4 p-4 grid grid-cols-4 gap-4")
                .add_children(squares),
        );
    let page = HtmlPage::new()
        .title("simple")
        .add_header_child(script().src("https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"))
        .add_header_child(
            style()
                .r#type("text/tailwindcss")
                .inner("body { @apply bg-sky-950 text-slate-100; }"),
        )
        .add_body_child(body)
        .render();
    std::fs::write("examples_outputs/simple.html", page).unwrap();
    println!("Done!");
}
