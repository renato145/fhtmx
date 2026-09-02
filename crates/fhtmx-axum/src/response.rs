use axum::{
    extract::FromRequestParts,
    response::{IntoResponse, Response},
};
use fhtmx::prelude::Render;
use http::{HeaderName, header, request};

/// Renders fhtmx nodes into an Axum [`Response`](axum::response::Response).
///
/// # Examples
///
/// ```
/// use axum::response::Response;
/// use fhtmx::prelude::*;
/// use fhtmx_axum::prelude::FhtmxAxumResponse;
///
/// fn index() -> Response {
///     div().add("Hello, htmx!").render_response()
/// }
/// ```
pub trait FhtmxAxumResponse {
    /// Builds an HTML response.
    fn render_response(&self) -> Response;
}

impl<T: Render> FhtmxAxumResponse for T {
    fn render_response(&self) -> Response {
        let html_body = self.render();
        (
            [(header::CONTENT_TYPE, mime::TEXT_HTML_UTF_8.as_ref())],
            html_body,
        )
            .into_response()
    }
}

/// Extractor that reports whether the request was initiated by htmx
/// (the `hx-request` header is present).
///
/// # Examples
///
/// ```
/// use fhtmx_axum::response::HxRequest;
///
/// fn is_htmx(req: HxRequest) -> bool {
///     req.0
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct HxRequest(pub bool);

impl<S> FromRequestParts<S> for HxRequest
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        parts: &mut request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        const HX_REQUEST: HeaderName = HeaderName::from_static("hx-request");
        if parts.headers.contains_key(HX_REQUEST) {
            Ok(HxRequest(true))
        } else {
            Ok(HxRequest(false))
        }
    }
}
