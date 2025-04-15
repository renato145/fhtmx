use actix_web::{App, HttpResponse, HttpServer, http::header::ContentType, web};
use fhtmx::{elements::*, html_page::HtmlPage};
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

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

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await?;
    Ok(())
}

fn page_layout<C: HtmlRender + 'static>(title: &str, content: C) -> HttpResponse {
    let body = main_tag()
        .class("container mx-auto mt-4")
        .add_child(h1().inner(title).class("text-3xl font-bold text-center"))
        .add_child(content);
    let html_body = HtmlPage::new()
        .custom_html_node(html().set_attr("data-theme", "dark").set_attr("lang", "en"))
        .title(title)
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

#[tracing::instrument]
async fn index() -> HttpResponse {
    let children = div()
        .class("mt-8")
        .add_child(
            label()
                .class("flex items-center")
                .add_child(
                    span()
                        .class("text-lg font-medium")
                        .inner("Add items to the TODO list"),
                )
                .add_child(
                    input()
                        .class("ml-4 input")
                        .r#type("text")
                        .placeholder("Write your TODO here")
                        .autofocus()
                        .required(),
                ),
        )
        // TODO: complete
        .add_child(div().class("mt-4").inner("Put the todo result here..."));
    page_layout("Actix demo", children)
}
