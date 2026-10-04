---
name: fhtmx-axum-guidance
description: Use when writing or reviewing Axum handlers with fhtmx-axum — HTML responses via render_response(), the HxRequest extractor, FhtmxError as IntoResponse, out-of-band swaps, and Server-Sent Events (SseSetup/SseState). Pair with the fhtmx-guidance skill for the builder API.
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
use fhtmx_axum::prelude::*;                      // response helpers + result alias
use fhtmx_axum::sse::{SseSetup, SseState, SseHandlerQuery};
```

## Rendering responses

`FhtmxAxumResponse` is implemented for every `Render` type and produces an axum `Response`
with `text/html; charset=utf-8`:

```rust
use fhtmx_axum::prelude::*;

fn handler() -> Response {
    div().id("result").add("Hello, htmx!").render_response()
}
```

A partial handler renders just the fragment (no `HtmlPage`); a full page handler wraps the
fragment in `HtmlPage` (see `fhtmx-guidance`).

## Full page vs partial: HxRequest extractor

`HxRequest` is a tuple struct (`HxRequest(pub bool)`) implementing `FromRequestParts` —
destructure it directly in the handler signature:

```rust
use fhtmx_axum::prelude::HxRequest;

async fn index(HxRequest(is_htmx): HxRequest, State(state): State<AppState>) -> Response {
    if is_htmx {
        render_list(state).render_response() // partial
    } else {
        page_layout("Items", render_list(state)) // full page
    }
}
```

## Errors

`FhtmxError` implements `IntoResponse`: it responds with **HTTP 200** plus
`HX-Retarget`/`HX-Reswap` headers (default `#toast-container` / `afterbegin` when rendered
as a toast) so htmx always swaps the error into the page. It is traced via `tracing::error!`
unless `.skip_tracing()` was set.

Return `FhtmxAxumResult<T>` (= `Result<Response, FhtmxError>`) and attach context with
`fhtmx_context` / `into_fhtmx_error` from the `fhtmx` prelude:

```rust
async fn rm_todo(Path(id): Path<String>, State(state): State<AppState>) -> FhtmxAxumResult {
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
    State(state): State<AppState>,
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
- `.hx_swap_oob("true")` replaces the element **with the same id** in the page; the swapped
  element must carry the matching `id`.
- Error responses are HTTP 200 by design (see Errors); do not map `FhtmxError` to 4xx/5xx
  without an `hx-status` policy.
