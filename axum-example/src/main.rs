use axum::{
    Router,
    extract::State,
    response::Html,
    routing::{get, post},
};
use axum_example::{
    routes::{
        index::index_page,
        todo::{add_todo, clear_todo, get_todo, rm_todo, sort_todo, todo_form, update_todo},
    },
    state::AppState,
};
use fhtmx::prelude::*;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
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

    let state = Arc::new(AppState::default());

    let app = Router::new()
        .route("/", get(index_page))
        .route("/todo", post(add_todo))
        .route("/todo/{id}", get(get_todo).put(update_todo).delete(rm_todo))
        .route("/todo/{id}/form", get(todo_form))
        .route("/todo/sort", post(sort_todo))
        .route("/todo/clear", post(clear_todo))
        .route("/js_invoke", post(js_invoke))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[tracing::instrument(skip(state))]
async fn js_invoke(State(state): State<Arc<AppState>>) -> Html<String> {
    let n = state.todo_list.lock().unwrap().len();
    Html(iife(format!(r#"alert("Your list got {n} items.");"#)).render())
}
