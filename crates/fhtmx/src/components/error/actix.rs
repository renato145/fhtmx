use crate::components::FhtmxError;
use crate::htmx::HXSwap;
use crate::render::Render;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, ResponseError};

impl FhtmxError {
    pub fn render_actix_response(&self) -> HttpResponse {
        let html_body = self.as_element().render();
        let mut builder = HttpResponse::Ok();
        builder.content_type(ContentType::html());
        match (self.as_toast, &self.hx_retarget) {
            (_, Some(target)) => {
                builder.append_header(("HX-Retarget", target.as_str()));
            }
            (true, None) => {
                builder.append_header(("HX-Retarget", "#toast-container"));
            }
            _ => {}
        }
        match (self.as_toast, &self.hx_reswap) {
            (_, Some(hx_swap)) => {
                builder.append_header(("HX-Reswap", hx_swap.as_str()));
            }
            (true, None) => {
                builder.append_header(("HX-Reswap", HXSwap::AfterBegin.to_string()));
            }
            _ => {}
        }
        builder.body(html_body)
    }
}

impl ResponseError for FhtmxError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::OK
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        if self.do_trace {
            tracing::error!(
                error.cause_chain=?self, error.message=%self,
                "Failed to render FhtmxError."
            );
        }
        self.render_actix_response()
    }
}
