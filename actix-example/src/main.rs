use actix_example::{
    routes::{
        index::index_page,
        todo::{add_todo, clear_todo, get_todo, rm_todo, sort_todo, todo_form, update_todo},
    },
    state::AppState,
};
use actix_web::{App, HttpResponse, HttpServer, web};
use fhtmx::prelude::*;
use fhtmx_actix::prelude::*;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

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

    let state = web::Data::new(AppState::default());

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(index_page))
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

#[tracing::instrument(skip(state))]
async fn js_invoke(state: web::Data<AppState>) -> HttpResponse {
    let n = state.todo_list.lock().unwrap().len();
    iife(format!(r#"alert("Your list got {n} items.");"#)).render_response()
}
