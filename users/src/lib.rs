use actix_web::{error::ParseError, http::header::Header, HttpRequest};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use core_rs::{
    error::{ServiceError, ServiceErrorJSON},
    AuthenticateResult,
};
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

pub mod db;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: String,
    pub profile_picture: String,
}

impl User {
    pub fn new(username: String, profile_picture: String) -> Self {
        Self {
            username,
            profile_picture,
        }
    }
}

pub async fn get_bearer_token(req: &HttpRequest) -> Result<String, ServiceError> {
    let parsed_auth = Authorization::<Bearer>::parse(req);

    match parsed_auth {
        Ok(auth) => Ok(auth.into_scheme().token().to_string()),
        Err(ParseError::Header) => Err(ServiceError::AuthorizationHeaderError),
        Err(_) => Err(ServiceError::AuthenticationError),
    }
}

pub async fn authenticate(req: &HttpRequest, expected_username: &str) -> Result<(), ServiceError> {
    let session_token = get_bearer_token(req).await?;

    let client = reqwest::Client::new();
    let response = client
        .get("http://auth:8080/authenticate")
        .header(AUTHORIZATION, format!("Bearer {}", session_token))
        .send()
        .await
        .expect("Failed to make request to auth/authenticate");

    let status = response.status();

    let body = response.text().await.expect("Failed to read response body");

    if status != 200 {
        let service_error_serde = serde_json::from_str::<ServiceErrorJSON>(&body).unwrap();
        let service_error = ServiceError::from(service_error_serde);
        return Err(service_error);
    }

    let auth_result: AuthenticateResult = serde_json::from_str(&body).unwrap();

    if auth_result.username != expected_username {
        return Err(ServiceError::AuthenticationError);
    }

    Ok(())
}
