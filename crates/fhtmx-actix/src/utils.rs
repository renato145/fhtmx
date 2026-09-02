use actix_web::HttpResponse;
use fhtmx::prelude::FhtmxResult;

/// Alias for `Result<HttpResponse, FhtmxError>`
pub type FhtmxActixResult = FhtmxResult<HttpResponse>;
