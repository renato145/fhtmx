---
name: fhtmx-guidance
description: Use when writing or reviewing Rust code that builds HTML with the fhtmx crate — tag builders, Element methods, DaisyUI components (dc_*/mk_*), HtmlPage, the HtmlView derive, FhtmxError, htmx attributes, or SVG. Framework-agnostic; pair with fhtmx-actix-guidance or fhtmx-axum-guidance for web-handler wiring.
metadata:
  fhtmx: "0.32"
---

# fhtmx Guidance

fhtmx is a Rust HTML builder with first-class htmx attribute support and DaisyUI components.
Build a tree of nodes with tag functions and chainable builder methods, then render to HTML.

**Scope:** the framework-agnostic builder API (crate `fhtmx`).

- For HTTP handler wiring (responses, extractors, SSE), load `fhtmx-actix-guidance` or
  `fhtmx-axum-guidance` — whichever the project uses — if installed.
- For htmx semantics in depth (hx-trigger expressions, events, JS API), load the
  `htmx-guidance` skill if available, otherwise see <https://four.htmx.org>.
- Verify symbols against <https://docs.rs/fhtmx> when a name is missing; this skill targets
  the fhtmx version listed in `metadata`.

## Mental model

```rust
use fhtmx::prelude::*;

let html = div()
    .class("container")
    .add(h1().add("Hello, world!"))      // children: any IntoNode
    .add(p().add("Some text"))
    .render();                           // Render trait -> String
```

- Tag functions (`div()`, `p()`, …) return an `HtmlElement`; chain builder methods, finish
  with `.render()`.
- Rendering is pretty-printed: block-level children are indented (2 spaces), inline content
  stays on one line, void tags render `<br />`.
- Text and attribute values are HTML-escaped on render. `add_raw`/`set_raw_attr` are the
  explicit unescaped escape hatches.

## Import the prelude

```rust
use fhtmx::prelude::*;
```

This re-exports every tag function, the `Element`/`Render`/`IntoNode` traits, components,
`HtmlPage`, htmx enums, sources, and the `HtmlView` derive. Crates that only need the derive
can `use fhtmx_derive::HtmlView` directly.

## Builder API (Element trait)

Consuming methods return `Self`; every one has a `_mut` variant for in-place building
(`add`/`add_mut`, `set_attr`/`set_attr_mut`, …). Prefer the consuming form when chaining,
the `_mut` form when building conditionally (loops, `match` arms).

### Children

| Method | Behavior |
|---|---|
| `.add(node)` / `.add_mut(node)` | Append child (any `IntoNode`). Fragments passed here are flattened into the parent's children. |
| `.add_opt(node)` | Append only if `Some`. |
| `.add_children(iter)` | Append from iterator (any `IntoIterator<Item: IntoNode>`). Fragments are kept as fragments. |
| `.add_opt_children(iter)` | Append from `Option<impl IntoIterator>`. |
| `.insert_child(i, node)` / `.prepend_child(node)` | Positional insert, with `opt` variants. |
| `.add_raw(html)` | Append **unescaped** HTML (also `raw_node()`). |
| `.update_html_element(i, f)` | Transform the child element at index `i` with a closure. |

`Vec<T: IntoNode>` implements `IntoNode` (renders as a fragment without a wrapper), so
`vec![a, b].into_node()` or `children!["text", p(), 42]` are common for batch responses.
`fragment([a, b])` builds one directly.

**Style rule:** chain at most two `.add()` calls — for three or more children use
`.add_children([...])`:

```rust
// prefer this over chaining .add(a).add(b).add(c):
dc_stat().add_children([
    dc_stat_title().add(title),
    dc_stat_value().add(value),
    dc_stat_desc().add(desc),
])
```

Array literals are homogeneous (one element type): when children mix elements and nodes,
build the list with `children![...]` instead.

### Classes

| Method | Behavior |
|---|---|
| `.class("a b")` | **Replaces all** classes with the given (space-separated) classes. |
| `.add_class("mx-4")` | Appends (space-separated supported). Use this on components so their preset class survives. |
| `.add_opt_class(Some("…"))` / `.remove_class()` / `.toggle_class()` / `.has_class()` | Conditional, removal, toggle, membership. |

### Attributes

| Method | Behavior |
|---|---|
| `.set_attr("name", value)` | Set any attribute; value may be any `IntoAttributeValue`. |
| `.set_opt_attr("id", Some(v))` | Set only if `Some`. |
| `.set_empty_attr("hidden")` | Boolean (valueless) attribute. |
| `.set_opt_empty_attr(Some("hidden"))` | Boolean attribute only if `Some`. |
| `.set_raw_attr("name", v)` | Set **unescaped** value (escape hatch). |

`IntoAttributeValue` is implemented for all `Display` types plus `bool`, where `true` renders
an empty attribute and `false` omits it entirely — this makes conditional toggles concise:

```rust
input().typ("text").set_attr("disabled", !enabled).set_attr("checked", checked)
```

## Naming conventions

| Pattern | Meaning | Examples |
|---|---|---|
| tag fn | lowercase HTML tag name | `div()`, `table()`, `textarea()` |
| `svg_*` | SVG tags | `svg()`, `svg_path()`, `svg_circle()` |
| keyword aliases | avoid Rust keywords | `typ()` = `type`, `for_()` = `for`, `r#async`, `main_tag()` = `<main>` |
| `hx_partial()` | `<hx-partial>` element for multi-target responses | |
| camelCase attrs | snake_case fn renders camelCase | `view_box("viewBox")`, `stroke_width`, `aria_label` |
| `dc_*` | wraps a single DaisyUI class | `dc_btn()` = `button().class("btn")` |
| `mk_*` | composed components | `mk_card()`, `mk_alert_error()` |
| `source_*`, `daisy_link()`, `typography_css()` | CDN `<script>`/`<link>` for `<head>` | `source_htmx()`, `source_tailwind()` |
| `script_setup_*` | bundled behavior scripts | `script_setup_sse()`, `script_setup_toast()`, `script_setup_theme()` |

`set_attr` works for anything without a typed setter, including htmx inline handlers:

```rust
form().set_attr("hx-on::after:request", "this.reset()")
```

## htmx attributes

All htmx setters live on `HtmlElement`, take one value (strings/enums — htmx attrs are
never valueless), and are HTML-escaped (JSON/HCON values round-trip fine — the browser
decodes attribute values before htmx reads them). Do **not** switch to `set_raw_attr` for
JSON; escaping is intentional and safe.

Requests: `hx_get`, `hx_post`, `hx_put`, `hx_patch`, `hx_delete`, `hx_query` (QUERY),
`hx_confirm`, `hx_vals` (JSON params), `hx_headers`, `hx_include`, `hx_sync`, `hx_boost`,
`hx_validate`, `hx_config`; `hx_method`/`hx_action` pair as an alternative to `hx_get`+method.

Targeting & swapping: `hx_target(HXTarget)`, `hx_swap(HXSwap)`, `hx_swap_oob`,
`hx_select`, `hx_select_oob`, `hx_preserve`, `hx_indicator`, `hx_disable`, `hx_ignore`,
`hx_history_elt`, `hx_morph_skip`, `hx_morph_skip_children`, `hx_trigger`.

```rust
use fhtmx::prelude::*;

button().hx_post("/items/7").hx_target(HXTarget::Closest("li")).hx_swap(HXSwap::OuterHTML);
div().hx_get("/search").hx_trigger("keyup[key=='Enter'] changed");
li().hx_delete("/items/7").hx_confirm("Are you sure?");
form().hx_post("/register").hx_status("422", "target:#errors");
```

- `HXSwap` variants: `InnerHtml` (default), `OuterHTML`, `InnerMorph`, `OuterMorph`,
  `OuterSync`, `TextContent`, `BeforeBegin`, `AfterBegin`, `BeforeEnd`, `AfterEnd`, `Delete`,
  `None`. Swap modifiers pass as strings: `.hx_swap("innerHTML swapEmpty:true")`.
- `HXTarget` variants: `This`, `Closest("css")`, `Next`, `NextSelector("css")`, `Previous`,
  `PreviousSelector("css")`, `Find`. For `find <selector>` pass the string directly:
  `.hx_target("find .row")`.
- Attribute modifiers have no typed setters — use `set_attr`:
  `.set_attr("hx-target:inherited", "#output")`, `.set_attr("hx-swap:append", "true")`.
- `hx_status(code, value)` changes target/swap/history for a specific status code
  (`"404"`, `"50x"`, `"5xx"`): `.hx_status("422", "target:#errors")`.
- SSE/WebSocket: `hx_sse_connect`, `hx_sse_close`, `hx_ws_connect`, `hx_ws_send` — require
  the extension scripts (`source_htmx_sse()` / `source_htmx_ws()`).

## DaisyUI components

### `dc_*` — single-class wrappers

`dc_x()` returns the natural element pre-classed with the DaisyUI class (some preset extra
attributes, e.g. `dc_checkbox()` is an `input typ("checkbox")`). Common ones:

- Actions: `dc_btn`, `dc_dropdown`, `dc_dropdown_content`, `dc_modal*`, `dc_swap*`, `dc_fab*`
- Data display: `dc_badge`, `dc_card`, `dc_card_title`, `dc_card_body`, `dc_card_actions`,
  `dc_list`, `dc_list_row`, `dc_table`, `dc_stat*`, `dc_avatar`, `dc_status`, `dc_timeline*`
- Navigation: `dc_menu`, `dc_navbar*`, `dc_tabs`, `dc_tab`, `dc_breadcrumbs`, `dc_dock`,
  `dc_steps`, `dc_join`
- Forms: `dc_input`, `dc_select`, `dc_textarea`, `dc_checkbox`, `dc_radio`, `dc_toggle`,
  `dc_label`, `dc_floating_label`, `dc_file_input`, `dc_range`, `dc_fieldset`,
  `dc_fieldset_legend`, `dc_validator`, `dc_validator_hint`
- Feedback: `dc_alert`, `dc_loading`, `dc_progress`, `dc_skeleton`, `dc_toast`, `dc_tooltip`
- Layout: `dc_divider`, `dc_drawer*`, `dc_hero*`, `dc_indicator*`, `dc_stack`

Always customize with `add_class` (never `class`), so the preset DaisyUI class survives:

```rust
dc_btn().add_class("btn-primary btn-sm").add("Save")
```

### `mk_*` — composites

```rust
mk_card(Some("Title"), p().add("content"))                 // card with optional title
mk_alert_error("Failed")                                   // alert_error/info/success/warning
mk_callout_tip(Some("hint"), content, /*collapse*/ true)   // note/warning/important/error/tip/caution
mk_fieldset_container("Personal data")                     // fieldset + legend
mk_labelled_input("Email", dc_input().typ("email"))        // label wrapping an input
mk_dropdown("open", items, "btn m-1", "bg-base-100 p-2")   // details/summary dropdown
mk_swap("ON", "OFF")                                       // toggle showing two contents
mk_accordion([(title, content)], "bg-base-100", "", "", None)
mk_markdown("# md")                                        // markdown -> div.prose (trusted input only!)
mk_container() / mk_centered_container() / main_container()
theme_toggle()                                             // needs script_setup_theme() in <head>
lazy_load(None)                                            // loading spinners (lazy_load_*)
```

### Colors and icons

`DaisyColor` (Primary, Secondary, Accent, Neutral, Info, Success, Warning, Error, Base100-300)
generates theme-aware Tailwind classes: `.bg()`, `.text()`, `.content()`, `.bg_content()`,
`.border()`, `.outline()`, `.ring()`, `.fill()`, `.stroke()` — e.g.
`DaisyColor::Error.bg_content()` = `"bg-error text-error-content"`.

`icons::*` returns inline `SvgElement`s (`menu`, `user`, `search`, `email`, `sun`, `moon`,
`edit`, …); size/style them with `.class("h-6 w-6")`.

## Full pages: HtmlPage and head sources

```rust
use fhtmx::prelude::*;

let page = HtmlPage::new()
    .custom_html_node(html().set_attr("data-theme", "dark").lang("en"))
    .title("My page")
    .description("optional meta description")
    .add_header_node(source_htmx())     // htmx 4 (pinned, with SRI)
    .add_header_node(daisy_link())      // DaisyUI 5 stylesheet
    .add_header_node(source_tailwind()) // Tailwind 4 browser build
    .add_body_node(main_container().add(h1().add("Hi")))
    .render();
```

| Head helper | Purpose |
|---|---|
| `source_htmx()` | htmx 4 script, pinned with SRI hash |
| `source_htmx_sse()` / `source_htmx_ws()` | extensions for `hx-sse:connect` / `hx-ws:connect` |
| `source_alpinejs()` / `source_alpinejs_persist()` | Alpine (persist goes **before** alpinejs) |
| `source_tailwind()` | Tailwind 4 browser build |
| `daisy_link()` | DaisyUI 5 stylesheet |
| `typography_css()` | prose/typography styles |
| `script_setup_sse()` | SSE session-id handshake (see framework skills) |
| `script_setup_toast()` | toast dismissal — required by `setup_toast`/`FhtmxError` toasts |
| `script_setup_theme(light, dark)` | theme switching — required by `theme_toggle()` |

Defaults: charset UTF-8 and viewport meta are preset; `.title()` optional.

## Partials and out-of-band swaps

htmx expects the server to return **HTML fragments**, not JSON. A partial handler renders
just the fragment: `li().add("row").render()` (response helpers in the framework skills).

Out-of-band swaps update multiple regions from one response: mark extra elements with
`.hx_swap_oob("true")` (replaces the element with the same id) or a selector such as
`.hx_swap_oob("afterbegin:#toast-container")`. Elements with `hx_swap_oob` are removed from
the normal swap flow. Batch pattern:

```rust
fragment([row, counter.hx_swap_oob("true")])
```

`hx_partial()` produces an `<hx-partial>` element to update several targets from one
response with explicit `hx-target`/`hx-swap` control.

## HtmlView derive

`#[derive(HtmlView)]` renders a named struct as a DaisyUI card (list or table layout):

```rust
#[derive(HtmlView)]
#[html_view(title = "User", mode = "table", color = "primary")]
struct User {
    name: String,
    #[html_view(alias = "Age", value_class = "italic")]
    age: u8,
    #[html_view(skip)]
    password: String,
    #[html_view(value_debug_pretty)]
    details: Details,
    contract: Option<String>, // None renders as "-"
}

user.render_view();          // String; or .html_view() -> HtmlNode
```

Struct attrs: `title` (string or expr), `mode` (`"list"` default | `"table"` |
`"table_right"`), `color` (DaisyUI color), `class` (expr, extra card classes), `mode_class`
(expr), `postproc` (fn name — receives the finished card element and returns it).

Field attrs: `skip`, `alias`, `value` (expr), `value_display`, `value_debug`,
`value_debug_pretty` (exactly one of `value*` allowed), `row_class`, `value_class`.

## Errors: FhtmxError

`FhtmxError` renders any failure as HTML. Default: a DaisyUI error **toast**; adapters send
HTTP **200** with `HX-Retarget: #toast-container` and `HX-Reswap: afterbegin` so htmx always
swaps it into the page. With a source chain it renders as a callout containing the
"Caused by" chain (`.hide_source()` to drop it).

```rust
use fhtmx::prelude::*;

// Attach context to any Result<T, E: Error> or Option<T>:
let n: i32 = "abc".parse().fhtmx_context("Failed to parse age")?;
let user = users.find(id).fhtmx_context("User not found")?;

// From anyhow::Error (default feature):
return Err(e).into_fhtmx_error();

// Customize the rendered response:
Err(FhtmxError::custom_error("Could not save")
    .disable_toast()          // render as alert/callout instead of toast
    .hx_retarget("#errors")   // custom retarget (implies no default toast retarget)
    .skip_tracing())
```

`FhtmxErrorExt` gives `fhtmx_*`-prefixed setters on both `FhtmxError` and
`Result<T, FhtmxError>` for use in `?`-heavy handlers. Handlers return
`Result<impl Response, FhtmxError>` — see the framework skills for the response types.

⚠️ htmx 4 swaps 4xx/5xx responses by default; fhtmx deliberately answers errors with 200 +
retarget headers. If you return error HTML with an error status instead, design it as swap
content or opt out with `hx-status:4xx="swap:none"`.

## Utilities

```rust
iife("alert(\"hi\")")                                  // <script>(async () => { … })();</script>
UrlBuilder::new("/items").push_query("q", "a & b").finish() // "/items?q=a+%26+b"
random_id("toast")                                     // "toast-<uuid>"
escape_html("<b>")                                     // explicit escaping
```

## Extending

Integrate custom types by implementing:
- `IntoNode` (for values usable as children) — return `HtmlNode::Text(self.to_string())` for
  display types, or build elements.
- `IntoAttributeValue` (for values usable as attributes) — return `Some(AttributeValue::Value(...))`.

## Pitfalls

1. `.class()` **replaces** all classes — on `dc_*`/`mk_*` components use `.add_class()`.
2. `add`/`add_child` flattens fragments into the parent, `add_children` keeps them; avoid
   empty text/`add_raw("")` nodes and empty fragments — they can corrupt rendered output
   (known issue, see the fhtmx repo TODO).
3. htmx setters escape values by design — JSON in `hx_vals`/`hx_headers` round-trips; don't
   "fix" this with `set_raw_attr`.
4. `mk_markdown` emits raw HTML from the markdown source — trusted input only; sanitize
   untrusted markdown upstream.
5. `FhtmxError` responding with HTTP 200 + `HX-Retarget` is intentional (htmx must swap the
   error into the page); do not change it to 4xx/5xx without an `hx-status` policy.
6. Prefer `IntoNode` over `ToString` at component boundaries so callers can pass elements,
   not just text.

## Verifying rendering in tests

Build the node, `.render()`, and assert on the string:

```rust
let html = div().class("flex").add(p().add("Hi")).render();
insta::assert_snapshot!(html, @"<div class=\"flex\"><p>Hi</p></div>");
// or with googletest: expect_that!(html, contains_substring("Hi"));
```

Intentional render changes require updating inline snapshots deliberately.
