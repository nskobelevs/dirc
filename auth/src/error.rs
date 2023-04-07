use std::result;

use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, Responder};
use serde::{ser::SerializeMap, Serialize};

/// A custom error type for this service.
pub enum AuthError {
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
}

impl AuthError {
    /// Get the status code for this error.
    pub fn status_code(&self) -> StatusCode {
        match self {
            AuthError::UserNotFound(_) => StatusCode::NOT_FOUND,
            AuthError::JsonParsingError(_) => StatusCode::BAD_REQUEST,
            AuthError::NotFound => StatusCode::NOT_FOUND,
            AuthError::UsernameTaken(_) => StatusCode::CONFLICT,
            AuthError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::InvalidPassword => StatusCode::UNAUTHORIZED,
            AuthError::AuthenticationError => StatusCode::UNAUTHORIZED,
        }
    }

    /// Get the error message for this error.
    pub fn error_message(&self) -> String {
        match self {
            AuthError::UserNotFound(name) => format!("User '{}' does not exist", name),
            AuthError::JsonParsingError(error_str) => error_str.to_owned(),
            AuthError::NotFound => "Page not found".to_string(),
            AuthError::UsernameTaken(username) => format!("Username '{}' is taken", username),
            AuthError::DatabaseError(error_str) => error_str.to_owned(),
            AuthError::InvalidPassword => "Invalid password".to_string(),
            AuthError::AuthenticationError => "Failed to authenticate user".to_string(),
        }
    }

    /// Get the error type as a string for this message
    pub fn error_type(&self) -> String {
        match self {
            AuthError::UserNotFound(_) => "UserNotFound".to_string(),
            AuthError::JsonParsingError(_) => "JsonParsingError".to_string(),
            AuthError::NotFound => "PageNotFound".to_string(),
            AuthError::UsernameTaken(_) => "UsernameTaken".to_string(),
            AuthError::DatabaseError(_) => "DatabaseError".to_string(),
            AuthError::InvalidPassword => "InvalidPassword".to_string(),
            AuthError::AuthenticationError => "AuthenticationError".to_string(),
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
impl Serialize for AuthError {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct ErrorInner {
            #[serde(rename = "type")]
            kind: String,
            message: String,
        }

        let mut map = serializer.serialize_map(Some(1))?;

        map.serialize_entry(
            "error",
            &ErrorInner {
                kind: self.error_type(),
                message: self.error_message(),
            },
        )?;

        map.end()
    }
}

/// A Result type that's limited to AuthError only
/// Implements REsponder so that it can be returned from a handler
pub enum Response<E> {
    Ok(E),
    Err(AuthError),
}

impl<T> From<Result<T, AuthError>> for Response<T> {
    fn from(res: Result<T, AuthError>) -> Self {
        match res {
            Ok(value) => Response::Ok(value),
            Err(err) => Response::Err(err),
        }
    }
}

impl From<mongodb::error::Error> for AuthError {
    fn from(error: mongodb::error::Error) -> Self {
        AuthError::DatabaseError(error.to_string())
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
