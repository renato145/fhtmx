use actix_web::HttpResponse;
use fhtmx::prelude::FhtmxResult;

/// Alias for `Result<HttpResponse, KshUiError>`
pub type FhtmxActixResult = FhtmxResult<HttpResponse>;
