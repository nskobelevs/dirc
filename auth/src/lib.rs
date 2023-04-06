use serde::Deserialize;

pub mod error;

#[derive(Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}
