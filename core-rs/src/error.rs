use std::result;

use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, Responder};
use serde::{ser::SerializeMap, Deserialize, Serialize};

/// A custom error type for this service.
#[derive(Debug, PartialEq, Eq)]
pub enum ServiceError {
    /// The requested resource was not found. Custom 404 response to return a JSON error body
    NotFound,
    /// An error occurred while parsing the JSON body,
    JsonParsingError(String),
    /// The user was not found in the database.
    UserNotFound(String),
    /// Attempted to register a user with a username that is already taken.
    UsernameTaken(String),
    /// An error occurred while interacting with the database.
    DatabaseError(String),
    /// Attempted to login with an invalid password.
    InvalidPassword,
    /// Failed to verify the user's session token.
    AuthenticationError,
    /// Authorization header is missing
    AuthorizationHeaderError,
}

impl ServiceError {
    /// Get the status code for this error.
    pub fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::UserNotFound(_) => StatusCode::NOT_FOUND,
            ServiceError::JsonParsingError(_) => StatusCode::BAD_REQUEST,
            ServiceError::NotFound => StatusCode::NOT_FOUND,
            ServiceError::UsernameTaken(_) => StatusCode::CONFLICT,
            ServiceError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::InvalidPassword => StatusCode::UNAUTHORIZED,
            ServiceError::AuthenticationError => StatusCode::UNAUTHORIZED,
            ServiceError::AuthorizationHeaderError => StatusCode::BAD_REQUEST,
        }
    }

    /// Get the error message for this error.
    pub fn error_message(&self) -> String {
        match self {
            ServiceError::UserNotFound(name) => format!("User '{}' does not exist", name),
            ServiceError::JsonParsingError(error_str) => error_str.to_owned(),
            ServiceError::NotFound => "Page not found".to_string(),
            ServiceError::UsernameTaken(username) => format!("Username '{}' is taken", username),
            ServiceError::DatabaseError(error_str) => error_str.to_owned(),
            ServiceError::InvalidPassword => "Invalid password".to_string(),
            ServiceError::AuthenticationError => "Failed to authenticate user".to_string(),
            ServiceError::AuthorizationHeaderError => {
                "Authorization `Breaker` header is missing".to_string()
            }
        }
    }

    /// Get the error type as a string for this message
    pub fn error_type(&self) -> String {
        match self {
            ServiceError::UserNotFound(_) => "UserNotFound".to_string(),
            ServiceError::JsonParsingError(_) => "JsonParsingError".to_string(),
            ServiceError::NotFound => "PageNotFound".to_string(),
            ServiceError::UsernameTaken(_) => "UsernameTaken".to_string(),
            ServiceError::DatabaseError(_) => "DatabaseError".to_string(),
            ServiceError::InvalidPassword => "InvalidPassword".to_string(),
            ServiceError::AuthenticationError => "AuthenticationError".to_string(),
            ServiceError::AuthorizationHeaderError => "AuthorizationHeaderError".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ServiceErrorJSON {
    error: ServiceErrorInner,
}

#[derive(Serialize, Deserialize)]
struct ServiceErrorInner {
    #[serde(rename = "type")]
    kind: String,
    message: String,
}

impl ServiceErrorInner {
    fn get_username(&self) -> String {
        let start = self.message.find('\'').unwrap();
        let end = self.message.rfind('\'').unwrap();

        self.message[start + 1..end].to_string()
    }
}

impl From<ServiceErrorJSON> for ServiceError {
    fn from(error: ServiceErrorJSON) -> Self {
        match error.error.kind.as_str() {
            "UserNotFound" => ServiceError::UserNotFound(error.error.get_username()),
            "JsonParsingError" => ServiceError::JsonParsingError(error.error.message),
            "PageNotFound" => ServiceError::NotFound,
            "UsernameTaken" => ServiceError::UsernameTaken(error.error.get_username()),
            "DatabaseError" => ServiceError::DatabaseError(error.error.message),
            "InvalidPassword" => ServiceError::InvalidPassword,
            "AuthenticationError" => ServiceError::AuthenticationError,
            "AuthorizationHeaderError" => ServiceError::AuthorizationHeaderError,
            _ => ServiceError::NotFound,
        }
    }
}

/// Custom Serde implementation to serialize the error type and message as an object.
/// For JSON returns the following structure:
///
/// ```json
/// {
///     "error": {
///         "type": <TYPE>,
///         "message": <MESSAGE>
///     }
/// }
///
/// ```
impl Serialize for ServiceError {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;

        map.serialize_entry(
            "error",
            &ServiceErrorInner {
                kind: self.error_type(),
                message: self.error_message(),
            },
        )?;

        map.end()
    }
}

/// A Result type that's limited to AuthError only
/// Implements REsponder so that it can be returned from a handler
pub enum Response<T> {
    Ok(T),
    Err(ServiceError),
}

impl<T> From<Result<T, ServiceError>> for Response<T> {
    fn from(res: Result<T, ServiceError>) -> Self {
        match res {
            Ok(value) => Response::Ok(value),
            Err(err) => Response::Err(err),
        }
    }
}

impl From<mongodb::error::Error> for ServiceError {
    fn from(error: mongodb::error::Error) -> Self {
        ServiceError::DatabaseError(error.to_string())
    }
}

impl<E> Responder for Response<E>
where
    E: Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match self {
            Response::Ok(e) => HttpResponse::Ok().json(e),
            Response::Err(e) => HttpResponse::build(e.status_code()).json(e),
        }
    }
}
