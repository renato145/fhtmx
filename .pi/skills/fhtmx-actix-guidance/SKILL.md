---
name: fhtmx-actix-guidance
description: Use when writing or reviewing Actix-web handlers with fhtmx-actix — HTML responses via render_response(), the HXRequest extractor, FhtmxError as ResponseError, out-of-band swaps, and Server-Sent Events (SseSetup/SseState). Pair with the fhtmx-guidance skill for the builder API.
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

async fn index(req: HXRequest, state: web::Data<State>) -> HttpResponse {
    if req.is_htmx() {
        render_list(state).render_response() // partial
    } else {
        page_layout("Items", render_list(state)) // full page
    }
}
```

## Errors

`FhtmxError` implements `ResponseError`: it responds with **HTTP 200** plus
`HX-Retarget`/`HX-Reswap` headers (default `#toast-container` / `afterbegin` when rendered
as a toast) so htmx always swaps the error into the page. It is traced via `tracing::error!`
unless `.skip_tracing()` was set.

Return `FhtmxActixResult<T>` (= `Result<HttpResponse, FhtmxError>`) and attach context with
`fhtmx_context` / `into_fhtmx_error` from the `fhtmx` prelude:

```rust
async fn rm_todo(id: web::Path<String>, state: web::Data<State>) -> FhtmxActixResult {
    let id = Uuid::from_str(&id).fhtmx_context("invalid id")?;
    let item = state.get_item(id).fhtmx_context("id not found")?;
    Ok(item.html().render_response())
}
```

Customize error rendering with `FhtmxError::custom_error(...).disable_toast()`
(alert/callout instead of toast), `.hx_retarget("#errors")`, `.hide_source()`. htmx 4 swaps
4xx/5xx responses by default — keep the 200-status behavior unless you add an `hx-status`
policy.

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
    state: web::Data<State>,
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
- Error responses are HTTP 200 by design (see Errors); do not map `FhtmxError` to 4xx/5xx
  without an `hx-status` policy.
