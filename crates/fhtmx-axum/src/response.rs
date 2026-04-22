use axum::{
    extract::FromRequestParts,
    response::{IntoResponse, Response},
};
use fhtmx::prelude::Render;
use http::{HeaderName, header, request};

pub trait FhtmxAxumResponse {
    /// Build a Html response
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

/// Always “true” on htmx requests.
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
