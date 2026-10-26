use crate::{
    html_elements::{sort_list_btn, todo_list_description},
    state::AppState,
};
use axum::{extract::State, response::Html};
use fhtmx::prelude::*;
use std::sync::Arc;

fn page_layout(title: &str, content: impl IntoNode) -> Html<String> {
    let body = main_container()
        .add_class("mt-4")
        .add(h1().class("text-3xl font-bold text-center").add(title))
        .add(content);
    let html_body = HtmlPage::new()
        .custom_html_node(html().set_attr("data-theme", "dark").lang("en"))
        .add_header_node(source_htmx())
        .add_header_node(daisy_link())
        .add_header_node(source_tailwind())
        .add_header_node(script_setup_toast())
        .title(title)
        .add_body_node(body)
        .add_body_node(dc_toast().id("toast-container"))
        .render();
    Html(html_body)
}

#[tracing::instrument(skip(state))]
pub async fn index_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let items = state.read_todo_list();
    let todo_list = div().class("mt-4 p-4 flex flex-col gap-y-2").add_children([
        todo_list_description(items.len()),
        sort_list_btn(items.len()),
        ul().id("todo-list")
            .class("mt-2 list-inside list-disc")
            .add_children(items),
    ]);
    let page = div()
        .class("mt-8")
        .add(
            form()
                .class("flex flex-wrap gap-2")
                .hx_post("/todo")
                .hx_target("#todo-list")
                .hx_swap(HXSwap::BeforeEnd)
                .set_attr("hx-on::after:request", "this.reset()")
                .add_children([
                    p().class("basis-full text-lg font-medium")
                        .add("Add items to the TODO list")
                        .add(
                            span()
                                .class("ml-2 text-sm label")
                                .add("(Punctuation symbols are not allowed)"),
                        ),
                    dc_input()
                        .typ("text")
                        .name("content")
                        .placeholder("Write your TODO here")
                        .autofocus()
                        .required(),
                    dc_btn().add_class("btn-primary").add("Add"),
                ]),
        )
        .add(todo_list);
    page_layout("Axum demo", page)
}
