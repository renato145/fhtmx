use actix_web::{
    App, HttpResponse, HttpServer,
    error::{ErrorBadRequest, ErrorInternalServerError},
    http::header::ContentType,
    web,
};
use fhtmx::prelude::*;
use fhtmx_actix::prelude::*;
use serde::Deserialize;
use std::{str::FromStr, sync::Mutex};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = std::env::args()
        .nth(1)
        .map(|x| x.parse::<u16>().unwrap())
        .unwrap_or(8000u16);

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
            .route("/js_invoke", web::post().to(js_invoke))
            .app_data(state.clone())
    })
    .bind(("0.0.0.0", port))?
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

    fn html(&self) -> HtmlElement {
        li().id(self.id.to_string())
            .add(&self.value)
            .add(
                dc_link()
                    .add_class("ml-2 link-info text-xs")
                    .hx_get(format!("/todo/{}/form", self.id))
                    .hx_target(HXTarget::Closest("li"))
                    .hx_swap(HXSwap::OuterHTML)
                    .add("modify"),
            )
            .add(
                dc_link()
                    .add_class("ml-1 link-error text-xs")
                    .hx_delete(format!("/todo/{}", self.id))
                    .hx_target(HXTarget::Closest("li"))
                    .hx_swap(HXSwap::Delete)
                    .hx_confirm("Are you sure?")
                    .add("remove"),
            )
    }

    fn html_form(&self) -> HtmlElement {
        li().class("my-2").id(self.id.to_string()).add(
            form()
                .class("contents")
                .add(
                    dc_input()
                        .typ("text")
                        .name("content")
                        .value(&self.value)
                        .set_attr("onfocus", "this.select()")
                        .autofocus()
                        .required(),
                )
                .add(
                    dc_btn()
                        .add_class("ml-2 btn-primary")
                        .hx_put(format!("/todo/{}", self.id))
                        .hx_target(HXTarget::Closest("li"))
                        .hx_swap(HXSwap::OuterHTML)
                        .add("ok"),
                )
                .add(
                    dc_btn()
                        .add_class("ml-1 btn-error")
                        .hx_get(format!("/todo/{}", self.id))
                        .hx_target(HXTarget::Closest("li"))
                        .hx_swap(HXSwap::OuterHTML)
                        .add("cancel"),
                ),
        )
    }
}

fn page_layout(title: &str, content: impl IntoNode) -> HttpResponse {
    let body = main_container()
        .add_class("mt-4")
        .add(h1().class("text-3xl font-bold text-center").add(title))
        .add(content);
    let html_body = HtmlPage::new()
        .custom_html_node(html().set_attr("data-theme", "dark").lang("en"))
        .add_header_node(source_htmx())
        .add_header_node(daisy_link())
        .add_header_node(source_tailwind())
        .title(title)
        .add_body_node(body)
        .render();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html_body)
}

fn todo_list_description(n: usize) -> HtmlElement {
    let s = match n {
        0 => "The list is empty, try adding some items...".to_string(),
        1 => "1 item".to_string(),
        n => format!("{} items:", n),
    };
    p().id("todo-list-description").add(s)
}

fn sort_list_btn(n: usize) -> HtmlElement {
    if n == 0 {
        div().id("sort-btn").hidden()
    } else {
        div()
            .id("sort-btn")
            .class("flex gap-x-2")
            .add(
                dc_btn()
                    .add_class("self-start btn-primary btn-sm")
                    .hx_post("/todo/sort")
                    .hx_target("#todo-list")
                    .add("Sort items"),
            )
            .add(
                dc_btn()
                    .add_class("self-start btn-error btn-sm")
                    .hx_post("/todo/clear")
                    .hx_target("#todo-list")
                    .hx_confirm("Are you sure?")
                    .add("Clear todo"),
            )
            .add(
                dc_btn()
                    .add_class("self-start btn-info btn-sm")
                    .hx_post("/js_invoke")
                    .hx_target("body")
                    .hx_swap(HXSwap::BeforeEnd)
                    .add("Call js"),
            )
    }
}

#[tracing::instrument(skip_all)]
async fn index(state: web::Data<State>) -> HttpResponse {
    let items = state.read_todo_list();
    let mut todo_list = div()
        .class("mt-8 p-4 flex flex-col gap-y-2")
        .add(todo_list_description(items.len()))
        .add(sort_list_btn(items.len()));
    todo_list = todo_list.add(
        ul().id("todo-list")
            .class("mt-2 list-inside list-disc")
            .add_children(items.iter().map(|o| o.html())),
    );
    let page = div()
        .class("mt-8")
        .add(
            form()
                .class("flex items-center gap-x-2")
                .hx_post("/todo")
                .hx_target("#todo-list")
                .hx_swap(HXSwap::BeforeEnd)
                .set_attr("hx-on::after-request", "this.reset()")
                .add(
                    label()
                        .class("flex items-center")
                        .add(
                            span()
                                .class("text-nowrap text-lg font-medium")
                                .add("Add items to the TODO list"),
                        )
                        .add(
                            dc_input()
                                .add_class("ml-4")
                                .typ("text")
                                .name("content")
                                .placeholder("Write your TODO here")
                                .autofocus()
                                .required(),
                        ),
                )
                .add(dc_btn().add_class("btn-primary").add("Add")),
        )
        .add(todo_list);
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
        todo_list_description(todo_list.len()).hx_swap_oob("true"),
    ];
    if todo_list.len() == 1 {
        html_body.push(sort_list_btn(1).hx_swap_oob("true"));
    }
    html_body.into_node().render_response()
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
    let mut html_body = vec![todo_list_description(todo_list.len()).hx_swap_oob("true")];
    if todo_list.is_empty() {
        html_body.push(sort_list_btn(0).hx_swap_oob("true"));
    }
    Ok(html_body.into_node().render_response())
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
        .into_node()
        .render_response()
}

#[tracing::instrument(skip(state))]
async fn clear_todo(state: web::Data<State>) -> HttpResponse {
    state.todo_list.lock().unwrap().clear();
    fragment([
        todo_list_description(0).hx_swap_oob("true"),
        sort_list_btn(0).hx_swap_oob("true"),
    ])
    .render_response()
}

#[tracing::instrument(skip(state))]
async fn js_invoke(state: web::Data<State>) -> HttpResponse {
    let n = state.todo_list.lock().unwrap().len();
    iife(format!(r#"alert("Your list got {n} items.");"#)).render_response()
}
