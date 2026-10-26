use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
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

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidContent => StatusCode::UNPROCESSABLE_ENTITY,
            Self::InvalidUuid(_) => StatusCode::BAD_REQUEST,
            Self::IdNotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status_code(),
            Html(mk_alert_error(self.to_string()).setup_toast(true).render()),
        )
            .into_response()
    }
}
