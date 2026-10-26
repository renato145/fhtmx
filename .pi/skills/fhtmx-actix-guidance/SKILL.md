---
name: fhtmx-actix-guidance
description: Use when writing or reviewing Actix-web handlers with fhtmx-actix — HTML responses via render_response(), the HXRequest extractor, error responses via ResponseError, out-of-band swaps, and Server-Sent Events (SseSetup/SseState). Pair with the fhtmx-guidance skill for the builder API.
metadata:
  fhtmx-actix: "0.11"
  fhtmx: "0.32"
---

# fhtmx for Actix-web

Covers only the `fhtmx-actix` adapter. **Prerequisite:** the builder API lives in the
`fhtmx-guidance` skill — load it if not already in context. Symbols:
<https://docs.rs/fhtmx-actix>. If the project uses axum instead, load `fhtmx-axum-guidance`;
the adapters mirror each other but differ in extractor shape, return types, and SSE wiring.

## Setup

```toml
[dependencies]
fhtmx = "0.32"
fhtmx-actix = "0.11"
actix-web = "4"
```

`fhtmx-actix` enables the `actix` feature of `fhtmx` transitively; do not redeclare it.

```rust
use fhtmx::prelude::*;
use fhtmx_actix::prelude::*; // includes response + SSE helpers
```

## Rendering responses

`FhtmxActixRender` is implemented for every `Render` type and produces an
`HttpResponse` with `ContentType::html()`:

```rust
use fhtmx_actix::prelude::*;

fn handler() -> HttpResponse {
    div().id("result").add("Hello, htmx!").render_response()
}
```

A partial handler renders just the fragment (no `HtmlPage`); a full page handler wraps the
fragment in `HtmlPage` (see `fhtmx-guidance`).

## Full page vs partial: HXRequest extractor

`HXRequest` checks the `HX-Request` header and branches between full page and htmx partial:

```rust
use fhtmx_actix::prelude::HXRequest;

async fn index(req: HXRequest, state: web::Data<AppState>) -> HttpResponse {
    if req.is_htmx() {
        render_list(state).render_response() // partial
    } else {
        page_layout("Items", render_list(state)) // full page
    }
}
```

## Errors

Define a typed error enum and implement `ResponseError`. The example
(`actix-example/src/error.rs`) maps each variant to a status code and renders the message as
an OOB toast:

```rust
use actix_web::{HttpResponse, ResponseError, http::header::ContentType};
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

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::InvalidContent => actix_web::http::StatusCode::UNPROCESSABLE_ENTITY,
            Self::InvalidUuid(_) => actix_web::http::StatusCode::BAD_REQUEST,
            Self::IdNotFound => actix_web::http::StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::html())
            .body(mk_alert_error(self.to_string()).setup_toast(true).render())
    }
}
```

Handlers return `Result<HttpResponse, AppError>` and use `?` normally. Requires
`script_setup_toast()` in `<head>` and `dc_toast().id("toast-container")` in the body.
htmx 4 swaps 4xx/5xx responses by default; the OOB toast is handled separately, but add
`hx-status` on destructive actions if the main target must not swap on error.

## Out-of-band swaps

Update several page regions from one response: main target in the body, other regions marked
with `.hx_swap_oob("true")` (replace-by-id):

```rust
vec![
    new_item.html(),
    todo_description(len).hx_swap_oob("true"),
]
    .into_node() // Vec<HtmlNode> -> fragment
    .render_response()
```

## Server-Sent Events

Server pushes rendered-HTML strings; clients swap them via the `hx-sse` extension.

Setup (in the app factory):

```rust
let sse_setup = SseSetup::new();          // or SseSetup::new_with_data::<T>()
let sse_data = sse_setup.state_data();    // web::Data<SseState<...>>

App::new()
    .route("/", web::get().to(index))
    .configure(|cfg| sse_setup.setup_route("/sse", cfg))
    .app_data(sse_data)
```

Producer handler (spawns a task; sends rendered HTML):

```rust
use actix_web_lab::sse::Data;

async fn start_stream(
    web::Query(query): web::Query<SseHandlerQuery>,   // sse_id: Uuid from setup_sse.js
    state: web::Data<AppState>,
    sse_state: web::Data<SseState<FhtmxUiNoSessionData>>,
) -> HttpResponse {
    let id = query.sse_id;
    tokio::spawn(async move {
        let data = Data::new(status_badge().render());
        sse_state.send_message(id, data);
        // ... sse_state.broadcast(data) / broadcast_all_but(id, data)
    });
    HttpResponse::Ok().finish()
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

- The extractor is `HXRequest` with a method (`.is_htmx()`), unlike the axum adapter's
  tuple-struct `HxRequest(pub bool)` — don't mix them.
- `.hx_swap_oob("true")` replaces the element **with the same id** in the page; the swapped
  element must carry the matching `id`.
- Error responses use real 4xx/5xx statuses and render an OOB toast (see Errors); use
  `hx-status` when the main target must not swap on error.
