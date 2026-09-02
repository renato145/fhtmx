use axum::response::Response;
use fhtmx::prelude::FhtmxResult;

/// Alias for `Result<Response, FhtmxError>`
pub type FhtmxAxumResult = FhtmxResult<Response>;
