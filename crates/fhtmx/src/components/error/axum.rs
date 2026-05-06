use crate::{components::FhtmxError, htmx::HXSwap, render::Render};
use axum_core::response::{IntoResponse, Response};
use http::{HeaderMap, header};

impl FhtmxError {
    pub fn render_axum_response(&self) -> Response {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            mime::TEXT_HTML_UTF_8.as_ref().parse().unwrap(),
        );
        match (self.as_toast, &self.hx_retarget) {
            (_, Some(target)) => {
                headers.insert("HX-Retarget", target.parse().unwrap());
            }
            (true, None) => {
                headers.insert("HX-Retarget", "#toast-container".parse().unwrap());
            }
            _ => {}
        }
        match (self.as_toast, &self.hx_reswap) {
            (_, Some(hx_swap)) => {
                headers.insert("HX-Reswap", hx_swap.parse().unwrap());
            }
            (true, None) => {
                headers.insert("HX-Reswap", HXSwap::AfterBegin.to_string().parse().unwrap());
            }
            _ => {}
        }
        let html_body = self.as_element().render();
        (headers, html_body).into_response()
    }
}

impl IntoResponse for FhtmxError {
    fn into_response(self) -> Response {
        if self.do_trace {
            tracing::error!(
                error.cause_chain=?self, error.message=%self,
                "Failed to render FhtmxError."
            );
        }
        self.render_axum_response()
    }
}
