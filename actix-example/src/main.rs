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
            .route("/todo/{id}/form", web::get().to(todo_form))
            .route("/todo/{id}", web::get().to(get_todo))
            .route("/todo/{id}", web::put().to(update_todo))
            .route("/todo/sort", web::post().to(sort_todo))
            .route("/todo/clear", web::post().to(clear_todo))
            .app_data(state.clone())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await?;
    Ok(())
}

#[derive(Default)]
struct State {
    todo_list: Mutex<Vec<TodoListItem>>,
}

impl State {
    fn read_todo_list(&self) -> Vec<TodoListItem> {
        self.todo_list.lock().unwrap().clone()
    }

    fn get_item(&self, id: Uuid) -> Option<TodoListItem> {
        self.todo_list
            .lock()
            .unwrap()
            .iter()
            .find(|o| o.id == id)
            .cloned()
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
                    .inner("modify")
                    .hx_get(format!("/todo/{}/form", self.id))
                    .hx_target(HXTarget::Closest("li"))
                    .hx_swap(HXSwap::OuterHTML)
                    .class("ml-2 link link-info text-xs"),
            )
            .add_child(
                button()
                    .inner("remove")
                    .hx_delete(format!("/todo/{}", self.id))
                    .hx_target(HXTarget::Closest("li"))
                    .hx_swap(HXSwap::Delete)
                    .hx_confirm("Are you sure?")
                    .class("ml-1 link link-error text-xs"),
            )
            .boxed()
    }

    fn html_form(&self) -> HtmlSingleElement {
        li().class("my-2")
            .id(self.id)
            .add_child(
                form()
                    .class("contents")
                    .add_child(
                        input()
                            .class("input")
                            .r#type("text")
                            .name("content")
                            .value(&self.value)
                            .set_attr("onfocus", "this.select()")
                            .autofocus()
                            .required(),
                    )
                    .add_child(
                        button()
                            .inner("ok")
                            .hx_put(format!("/todo/{}", self.id))
                            .hx_target(HXTarget::Closest("li"))
                            .hx_swap(HXSwap::OuterHTML)
                            .class("ml-2 btn btn-primary"),
                    )
                    .add_child(
                        button()
                            .inner("cancel")
                            .hx_get(format!("/todo/{}", self.id))
                            .hx_target(HXTarget::Closest("li"))
                            .hx_swap(HXSwap::OuterHTML)
                            .class("ml-1 btn btn-error"),
                    ),
            )
            .boxed()
    }
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
    p().id("todo-list-description").inner(&inner)
}

fn sort_list_btn(n: usize) -> HtmlElement<&'static str, HtmlGenericElement> {
    if n == 0 {
        div().id("sort-btn").hidden()
    } else {
        div()
            .id("sort-btn")
            .class("flex gap-x-2")
            .add_child(
                button()
                    .hx_post("/todo/sort")
                    .hx_target("#todo-list")
                    .class("self-start btn btn-primary btn-sm")
                    .inner("Sort items"),
            )
            .add_child(
                button()
                    .hx_post("/todo/clear")
                    .hx_target("#todo-list")
                    .hx_confirm("Are you sure?")
                    .class("self-start btn btn-error btn-sm")
                    .inner("Clear todo"),
            )
    }
}

trait HtmlActixRender: HtmlRender {
    fn render_response(&self) -> HttpResponse;
}

impl<T: ?Sized + HtmlRender> HtmlActixRender for T {
    fn render_response(&self) -> HttpResponse {
        let html_body = self.render();
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(html_body)
    }
}

#[tracing::instrument(skip_all)]
async fn index(state: web::Data<State>) -> HttpResponse {
    let items = state.read_todo_list();
    let mut todo_list = div()
        .class("mt-8 p-4 flex flex-col gap-y-2")
        .add_child(todo_list_description(items.len()))
        .add_child(sort_list_btn(items.len()));
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
                .class("flex items-center gap-x-2")
                .add_child(
                    label()
                        .class("flex items-center")
                        .add_child(
                            span()
                                .class("text-nowrap text-lg font-medium")
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
                )
                .add_child(button().class("btn btn-primary").inner("Add")),
        )
        .add_child(todo_list);
    page_layout("Actix demo", page)
}

#[derive(Debug, Deserialize)]
struct TodoContent {
    content: String,
}

#[tracing::instrument(skip(state))]
async fn add_todo(
    web::Form(params): web::Form<TodoContent>,
    state: web::Data<State>,
) -> HttpResponse {
    let new_item = TodoListItem::new(params.content);
    let mut todo_list = state.todo_list.lock().unwrap();
    todo_list.push(new_item.clone());
    let mut html_body = vec![
        new_item.html(),
        todo_list_description(todo_list.len())
            .hx_swap_oob("true")
            .boxed(),
    ];
    if todo_list.len() == 1 {
        html_body.push(sort_list_btn(1).hx_swap_oob("true").boxed());
    }
    html_body.render_response()
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
    let mut html_body = vec![
        todo_list_description(todo_list.len())
            .hx_swap_oob("true")
            .boxed(),
    ];
    if todo_list.is_empty() {
        html_body.push(sort_list_btn(0).hx_swap_oob("true").boxed());
    }
    Ok(html_body.render_response())
}

#[tracing::instrument(skip(state))]
async fn todo_form(
    id: web::Path<String>,
    state: web::Data<State>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = Uuid::from_str(&id).map_err(ErrorInternalServerError)?;
    let response = state
        .get_item(id)
        .ok_or_else(|| ErrorBadRequest("id not found."))?
        .html_form()
        .render_response();
    Ok(response)
}

#[tracing::instrument(skip(state))]
async fn get_todo(
    id: web::Path<String>,
    state: web::Data<State>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = Uuid::from_str(&id).map_err(ErrorInternalServerError)?;
    let response = state
        .get_item(id)
        .ok_or_else(|| ErrorBadRequest("id not found."))?
        .html()
        .render_response();
    Ok(response)
}

#[tracing::instrument(skip(state))]
async fn update_todo(
    id: web::Path<String>,
    web::Form(params): web::Form<TodoContent>,
    state: web::Data<State>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = Uuid::from_str(&id).map_err(ErrorInternalServerError)?;
    let mut todo_list = state.todo_list.lock().unwrap();
    let item = todo_list
        .iter_mut()
        .find(|o| o.id == id)
        .ok_or_else(|| ErrorBadRequest("id not found."))?;
    item.value = params.content;
    Ok(item.html().render_response())
}

#[tracing::instrument(skip(state))]
async fn sort_todo(state: web::Data<State>) -> HttpResponse {
    let mut todo_list = state.todo_list.lock().unwrap();
    todo_list.sort_by_key(|o| o.value.clone());
    todo_list
        .iter()
        .map(|o| o.html())
        .collect::<Vec<_>>()
        .render_response()
}

#[tracing::instrument(skip(state))]
async fn clear_todo(state: web::Data<State>) -> HttpResponse {
    state.todo_list.lock().unwrap().clear();
    vec![
        todo_list_description(0).hx_swap_oob("true").boxed(),
        sort_list_btn(0).hx_swap_oob("true").boxed(),
    ]
    .render_response()
}
