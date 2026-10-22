use actix_web::{HttpResponse, ResponseError, http::header::ContentType};
use fhtmx::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Invalid content for a todo item.")]
    InvalidContent,
    #[error("Invalid uuid.")]
    InvalidUuid(#[source] uuid::Error),
    #[error("Id not found.")]
    IdNotFound,
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::html())
            .body(mk_alert_error(self.to_string()).setup_toast(true).render())
    }
}
