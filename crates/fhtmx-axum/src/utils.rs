use axum::response::Response;
use fhtmx::prelude::FhtmxResult;

/// Alias for `Result<Response, KshUiError>`
pub type FhtmxAxumResult = FhtmxResult<Response>;
