use actix_web::{
    HttpResponse,
    http::header::{
        ContentType, Header, HeaderName, HeaderValue, InvalidHeaderValue, TryIntoHeaderValue,
    },
};
use fhtmx::prelude::Render;

/// Renders fhtmx nodes into an Actix [`HttpResponse`].
///
/// # Examples
///
/// ```
/// use actix_web::HttpResponse;
/// use fhtmx::prelude::*;
/// use fhtmx_actix::prelude::FhtmxActixRender;
///
/// fn index() -> HttpResponse {
///     div().add("Hello, htmx!").render_response()
/// }
/// ```
pub trait FhtmxActixRender {
    /// Renders as a `ContentType::html()`
    fn render_response(&self) -> HttpResponse;
}

impl<T: Render> FhtmxActixRender for T {
    fn render_response(&self) -> HttpResponse {
        let html_body = self.render();
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(html_body)
    }
}

/// Extractor that checks if the `HX-Request` header is present.
///
/// Implementing [`Header`](actix_web::http::header::Header) also makes it usable as a request
/// extractor. Use [`is_htmx`](Self::is_htmx) to branch between full pages and htmx partials.
///
/// # Examples
///
/// ```
/// use fhtmx::prelude::*;
/// use fhtmx_actix::prelude::HXRequest;
///
/// fn is_htmx(req: HXRequest) -> bool {
///     req.is_htmx()
/// }
/// ```
pub struct HXRequest(bool);

impl HXRequest {
    /// Whether this request was initiated by htmx.
    pub fn is_htmx(&self) -> bool {
        self.0
    }
}

impl TryIntoHeaderValue for HXRequest {
    type Error = InvalidHeaderValue;

    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(&self.0.to_string())
    }
}

impl Header for HXRequest {
    fn name() -> HeaderName {
        HeaderName::from_static("HX-Request")
    }

    fn parse<M: actix_web::HttpMessage>(msg: &M) -> Result<Self, actix_web::error::ParseError> {
        let x = msg.headers().contains_key("HX-Request");
        Ok(Self(x))
    }
}
