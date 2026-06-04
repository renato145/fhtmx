# fhtmx

Rust HTML builder library with htmx support. Cargo workspace with 4 crates.

## Project

- **Language**: Rust (edition 2024)
- **Workspace root**: `Cargo.toml` with members `crates/*`
- **Crates**:
  - `fhtmx` — core HTML/htmx builder, components, `HtmlView` derive macro re-export
  - `fhtmx-derive` — proc-macro crate (`HtmlView` derive)
  - `fhtmx-actix` — Actix-web response helpers and SSE utilities
  - `fhtmx-axum` — Axum response helpers and SSE utilities
- **Examples**: `crates/fhtmx/examples/`
- **Features on `fhtmx`**: `anyhow` (default), `chrono_0_4`, `jiff_0_2`, `actix`, `axum`

## Commands

| Task | Command |
|------|---------|
| Check all targets | `cargo check --all-targets` |
| Clippy | `cargo clippy --all-targets` |
| Format check | `cargo fmt --all -- --check` |
| Test all | `cargo test` |
| Test single | `cargo test <test_name>` or `cargo test <module::test_name>` |
| Test one crate | `cargo test -p fhtmx` |
| Run example | `cargo run --example <name>` (from `crates/fhtmx/`) |
| Watch/loop | `bacon` (configs in root and `crates/fhtmx/`) |

- Tests use snapshot assertions via `insta` (inline snapshots).
- Husky + commitlint enforce conventional commits (`npm install` / `pnpm install` for Node dev deps).

## Code Style

- Standard `cargo fmt`; no custom `rustfmt.toml`.
- No custom `clippy.toml`; standard `cargo clippy --all-targets`.
- Prefer builder-pattern methods with owned `self` and `_mut` variants (e.g., `add()` / `add_mut()`).
- Use `Cow<'static, str>` for strings stored in elements.
- Use `IndexMap`/`IndexSet` to preserve insertion order of attrs/classes.
- Implement `IntoNode` and `IntoAttributeValue` for custom types to integrate with the builder.
- Macros are central: `children!`, `set_attr!`, `create_tag_fn!`, `set_htmx_attr!`, `set_empty_attr!`.
- Errors in proc macros use `darling` for derive parsing and return `TokenStream` errors via `e.write_errors().into()`.
- Feature-gate optional dependencies cleanly (e.g., `#[cfg(feature = "chrono_0_4")]`).

## Architecture

Core types and flow:

1. `HtmlNode` (enum) — Doctype, Raw, Text, Element, SvgElement, Fragment
2. `Element` (trait) — shared behavior for HTML/SVG: tag, attrs, classes, children, void/inline checks
3. `HtmlElement` (struct) — concrete element implementing `Element`
4. `Render` (trait) — `render()` → `String`, `render_to(buf, indent)`
5. `IntoNode` / `AsNode` — convert values into `HtmlNode`
6. `IntoAttributeValue` — convert values into `AttributeValue` (Empty, Raw, Value)

Key modules in `fhtmx`:
- `html_element.rs` — all HTML tag constructors (`div()`, `p()`, etc.) and `HtmlElement`
- `element.rs` — `Element` trait, builder methods (`class`, `set_attr`, `add`, `add_raw`, etc.)
- `render.rs` — `Render` impl; indentation logic for block vs inline content
- `node.rs` — `HtmlNode`, `IntoNode`, `AsNode`, `children!` macro
- `attribute.rs` — `AttributeValue`, `IntoAttributeValue`
- `htmx.rs` — htmx attribute setters and enums (`HXSwap`, `HXTarget`, request/response headers)
- `html_page.rs` — `HtmlPage` builder (doctype, head, body)
- `html_view.rs` — `HtmlView` trait and `html_list_row`; derive macro generates card/table/list views
- `components/` — DaisyUI wrappers, forms, markdown, alerts, toast, theme, lazy load, error components
- `svg.rs` — SVG element builder
- `js.rs` — JS snippet helpers

Integrations:
- `fhtmx-actix`: `response.rs` (Actix `Responder`), `sse.rs` (SSE streams), `utils.rs`
- `fhtmx-axum`: `response.rs` (Axum `IntoResponse`), `sse.rs` (SSE streams), `utils.rs`

`fhtmx-derive`:
- `HtmlView` derive reads `#[html_view(...)]` attributes on structs and fields using `darling`
- Supports modes: `List`, `Table`, `TableRight`
