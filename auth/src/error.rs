use std::result;

use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, Responder};
use serde::{ser::SerializeMap, Serialize};

/// A custom error type for this service.
pub enum AuthError {
    NotFound,
    JsonParsingError(String),
    UserNotFound(String),
    UsernameTaken(String),
    DatabaseError(String),
    InvalidPassword,
    AuthenticationError,
}

impl AuthError {
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

impl Serialize for AuthError {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct ErrorInner {
            #[serde(rename = "type")]
            type_: String,
            message: String,
        }

        let mut map = serializer.serialize_map(Some(1))?;

        map.serialize_entry(
            "error",
            &ErrorInner {
                type_: self.error_type(),
                message: self.error_message(),
            },
        )?;

        map.end()
    }
}

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
