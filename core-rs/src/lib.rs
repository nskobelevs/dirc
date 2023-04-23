use actix_web::{web, HttpResponse};
use error::ServiceError;
use serde::{Deserialize, Serialize};

pub mod error;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateResult {
    pub username: String,
}

impl From<String> for AuthenticateResult {
    fn from(username: String) -> Self {
        AuthenticateResult { username }
    }
}

pub fn create_json_cfg() -> web::JsonConfig {
    web::JsonConfig::default()
        .limit(4096)
        .content_type(|mime| mime == mime::TEXT_PLAIN || mime == mime::APPLICATION_JSON)
        .error_handler(|err, _req| {
            let error_str = err.to_string();

            actix_web::error::InternalError::from_response(
                err,
                HttpResponse::BadRequest().json(ServiceError::JsonParsingError(error_str)),
            )
            .into()
        })
}
