use actix_web::{
    App, HttpResponse, HttpServer,
    error::{ErrorBadRequest, ErrorInternalServerError},
    http::header::ContentType,
    web,
};
use fhtmx::prelude::*;
use serde::Deserialize;
use std::{str::FromStr, sync::Mutex};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[derive(Default)]
struct State {
    todo_list: Mutex<Vec<TodoListItem>>,
}

impl State {
    fn read_todo_list(&self) -> Vec<TodoListItem> {
        self.todo_list.lock().unwrap().clone()
    }
}

#[derive(Debug, Clone)]
struct TodoListItem {
    id: Uuid,
    value: String,
}

impl TodoListItem {
    fn new(value: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            value,
        }
    }

    fn html(&self) -> HtmlSingleElement {
        li().id(self.id)
            .inner(&self.value)
            .add_child(
                button()
                    .hx_delete(format!("/todo/{}", self.id))
                    .hx_target(HXTarget::Closest("li"))
                    .hx_swap(HXSwap::Delete)
                    .hx_confirm("Are you sure?")
                    .class("ml-2 link link-error text-xs")
                    .inner("remove"),
            )
            .boxed()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE),
        )
        .init();

    let state = web::Data::new(State::default());

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(index))
            .route("/todo", web::post().to(add_todo))
            .route("/todo/{id}", web::delete().to(rm_todo))
            .app_data(state.clone())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await?;
    Ok(())
}

fn page_layout<C: HtmlRender + 'static>(title: &str, content: C) -> HttpResponse {
    let body = main_tag()
        .class("container mx-auto mt-4")
        .add_child(h1().inner(title).class("text-3xl font-bold text-center"))
        .add_child(content);
    let html_body = HtmlPage::new()
        .custom_html_node(html().set_attr("data-theme", "dark").lang("en"))
        .title(title)
        .add_header_child(
            script()
                .src("https://unpkg.com/htmx.org@2.0.4")
                .set_attr(
                    "integrity",
                    "sha384-HGfztofotfshcF7+8n44JQL2oJmowVChPTg48S+jvZoztPfvwD79OC/LTtG6dMp+",
                )
                .set_attr("crossorigin", "anonymous"),
        )
        .add_header_child(
            link()
                .href("https://cdn.jsdelivr.net/npm/daisyui@5")
                .rel("stylesheet")
                .r#type("text/css"),
        )
        .add_header_child(script().src("https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"))
        .add_body_child(body)
        .render();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html_body)
}

fn todo_list_description(n: usize) -> HtmlElement<&'static str, HtmlGenericElement> {
    let inner = match n {
        0 => "The list is empty, try adding some items...".to_string(),
        1 => "1 item".to_string(),
        n => format!("{} items:", n),
    };
    p().id("todo-list-description").inner(inner)
}

#[tracing::instrument(skip_all)]
async fn index(state: web::Data<State>) -> HttpResponse {
    let items = state.read_todo_list();
    let mut todo_list = div()
        .class("mt-8 p-4")
        .add_child(todo_list_description(items.len()));
    todo_list = todo_list.add_child(
        ul().id("todo-list")
            .class("mt-2 list-inside list-disc")
            .add_children(items.iter().map(|o| o.html()).collect()),
    );
    let page = div()
        .class("mt-8")
        .add_child(
            form()
                .hx_post("/todo")
                .hx_target("#todo-list")
                .hx_swap(HXSwap::BeforeEnd)
                .set_attr("hx-on::after-request", "this.reset()")
                .add_child(
                    label()
                        .class("flex items-center")
                        .add_child(
                            span()
                                .class("text-lg font-medium")
                                .inner("Add items to the TODO list"),
                        )
                        .add_child(
                            input()
                                .class("ml-4 input")
                                .r#type("text")
                                .name("content")
                                .placeholder("Write your TODO here")
                                .autofocus()
                                .required(),
                        ),
                ),
        )
        .add_child(todo_list);
    page_layout("Actix demo", page)
}

#[derive(Debug, Deserialize)]
struct AddTodo {
    content: String,
}

#[tracing::instrument(skip(state))]
async fn add_todo(web::Form(params): web::Form<AddTodo>, state: web::Data<State>) -> HttpResponse {
    let new_item = TodoListItem::new(params.content);
    let mut todo_list = state.todo_list.lock().unwrap();
    todo_list.push(new_item.clone());
    let html_body = vec![
        new_item.html(),
        todo_list_description(todo_list.len())
            .hx_swap_oob("true")
            .boxed(),
    ]
    .render();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html_body)
}

#[tracing::instrument(skip(state))]
async fn rm_todo(
    id: web::Path<String>,
    state: web::Data<State>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = Uuid::from_str(&id).map_err(ErrorInternalServerError)?;
    let mut todo_list = state.todo_list.lock().unwrap();
    let idx = todo_list
        .iter()
        .position(|o| o.id == id)
        .ok_or_else(|| ErrorBadRequest("id not found."))?;
    todo_list.remove(idx);
    let html_body = todo_list_description(todo_list.len())
        .hx_swap_oob("true")
        .render();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html_body))
}
