---
name: fhtmx-axum-guidance
description: Use when writing or reviewing Axum handlers with fhtmx-axum — HTML responses via Html<String>, the HxRequest extractor, error responses via IntoResponse, out-of-band swaps, and Server-Sent Events (SseSetup/SseState). Pair with the fhtmx-guidance skill for the builder API.
metadata:
  fhtmx-axum: "0.4"
  fhtmx: "0.32"
---

# fhtmx for Axum

Covers only the `fhtmx-axum` adapter. **Prerequisite:** the builder API lives in the
`fhtmx-guidance` skill — load it if not already in context. Symbols:
<https://docs.rs/fhtmx-axum>. If the project uses actix-web instead, load
`fhtmx-actix-guidance`; the adapters mirror each other but differ in extractor shape, return
types, and SSE wiring.

## Setup

```toml
[dependencies]
fhtmx = "0.32"
fhtmx-axum = "0.4"
axum = "0.8"
```

`fhtmx-axum` enables the `axum` feature of `fhtmx` transitively; do not redeclare it.

```rust
use fhtmx::prelude::*;

// ⚠️ SSE types are NOT in the axum prelude — import them explicitly:
use fhtmx_axum::prelude::*;                      // HxRequest + response helpers
use fhtmx_axum::sse::{SseSetup, SseState, SseHandlerQuery};
```

## Rendering responses

Return typed HTML with `Html<String>`; axum sets `text/html; charset=utf-8`:

```rust
use axum::response::Html;
use fhtmx::prelude::*;

fn handler() -> Html<String> {
    Html(div().id("result").add("Hello, htmx!").render())
}
```

A partial handler renders just the fragment (no `HtmlPage`); a full page handler wraps the
fragment in `HtmlPage` (see `fhtmx-guidance`). The `FhtmxAxumResponse::render_response()`
helper still exists when an opaque `Response` is required, but prefer `Html<String>` in
normal handlers.

## Full page vs partial: HxRequest extractor

`HxRequest` is a tuple struct (`HxRequest(pub bool)`) implementing `FromRequestParts` —
destructure it directly in the handler signature:

```rust
use axum::{extract::State, response::Html};
use fhtmx_axum::prelude::HxRequest;
use std::sync::Arc;

async fn index(
    HxRequest(is_htmx): HxRequest,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    if is_htmx {
        Html(render_list(state).render()) // partial
    } else {
        page_layout("Items", render_list(state)) // full page -> Html<String>
    }
}
```

## Errors

Define a typed error enum and implement `IntoResponse`. The example
(`axum-example/src/error.rs`) maps each variant to a status code and renders the message as
an OOB toast:

```rust
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use fhtmx::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Invalid content for a todo item.")]
    InvalidContent,
    #[error("Invalid uuid.")]
    InvalidUuid(#[source] uuid::Error),
    #[error("Id not found.")]
    IdNotFound,
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidContent => StatusCode::UNPROCESSABLE_ENTITY,
            Self::InvalidUuid(_) => StatusCode::BAD_REQUEST,
            Self::IdNotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status_code(),
            Html(mk_alert_error(self.to_string()).setup_toast(true).render()),
        )
            .into_response()
    }
}
```

Handlers return `Result<Html<String>, AppError>` (or `Html<String>` on success) and use `?`
normally. Requires `script_setup_toast()` in `<head>` and
`dc_toast().id("toast-container")` in the body. htmx 4 swaps 4xx/5xx responses by default;
the OOB toast is handled separately, but add `hx-status` on destructive actions if the main
target must not swap on error.

## Out-of-band swaps

Update several page regions from one response: main target in the body, other regions marked
with `.hx_swap_oob("true")` (replace-by-id):

```rust
Html(
    vec![
        new_item.html(),
        todo_description(len).hx_swap_oob("true"),
    ]
    .into_node() // Vec<HtmlNode> -> fragment
    .render(),
)
```

## Server-Sent Events

Server pushes rendered-HTML **strings** (simpler than actix, which wraps in
`actix_web_lab::sse::Data`); clients swap them via the `hx-sse` extension.

Setup (in the router factory) — note the state is the plain `SseState<T>` (it is `Clone`),
not wrapped in `web::Data`:

```rust
let sse_setup = SseSetup::new();          // or SseSetup::new_with_data::<T>()
let sse_state = sse_setup.state_data();   // SseState<...> — pass with with_state

let app: Router<()> = Router::new()
    .route("/", get(index))
    .merge(sse_setup.sse_route("/sse"))
    .with_state(sse_state)
```

Producer handler (spawns a task; sends rendered HTML as a plain string):

```rust
async fn start_stream(
    Query(query): Query<SseHandlerQuery>,     // sse_id: Uuid from setup_sse.js
    State(state): State<Arc<AppState>>,
    sse_state: State<SseState<FhtmxUiNoSessionData>>,
) -> Response {
    let id = query.sse_id;
    tokio::spawn(async move {
        sse_state.send_message(id, status_badge().render());
        // ... sse_state.broadcast(html) / broadcast_all_but(id, html)
    });
    Response::default()
}
```

Client side (layout):

```rust
HtmlPage::new()
    .add_header_node(source_htmx_sse())   // hx-sse extension script
    .add_header_node(script_setup_sse())  // captures the sse_id event, tags requests
    // trigger a producer once the session id arrives:
    .hx_get("/start_stream").hx_trigger("sse_id once from:body").hx_swap(HXSwap::None)
    // consumer: swaps pushed HTML into #logs:
    .hx_sse_connect("/sse").hx_target("#logs").hx_swap(HXSwap::AfterBegin)
```

- Each connection registers a session under a fresh UUID; the server sends it as an
  `sse_id` event, `script_setup_sse()` stores it, and it is appended to subsequent request
  bodies (`sse_id` query/form param) and to `SseHandlerQuery` for producers.
- `send_message`/`broadcast` return `Option<()>`/`usize`; `None` means the session's channel
  is closed (session removed) — stop the producer when you get `None`.
- Sessions are removed when the channel closes; sends use `try_send` (capacity 8), so
  bursts can drop messages under load (known issue).

## Gotchas

- The extractor is `HxRequest(pub bool)` — access `.0` or destructure; unlike the actix
  adapter's `HXRequest`, there is no `.is_htmx()` method. Don't mix them.
- The axum prelude does **not** re-export the `sse` module (the actix prelude does) — always
  `use fhtmx_axum::sse::{SseSetup, SseState, SseHandlerQuery};` explicitly.
- Name the shared state `AppState`, not `State`: `axum::extract::State` is also in scope,
  and the examples use `State<Arc<AppState>>`.
- `.hx_swap_oob("true")` replaces the element **with the same id** in the page; the swapped
  element must carry the matching `id`.
- Error responses use real 4xx/5xx statuses and render an OOB toast (see Errors); use
  `hx-status` when the main target must not swap on error.
