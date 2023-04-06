use pbkdf2::pbkdf2_hmac_array;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

pub mod db;
pub mod error;

#[derive(Clone, Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SessionToken {
    token: String,
}

impl SessionToken {
    pub fn new() -> Self {
        let token = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>();

        SessionToken { token }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Default for SessionToken {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password_hash: String,
    salt: String,
}

impl Credentials {
    pub fn new(login_info: LoginInfo) -> Self {
        let salt = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>();

        let hashed_password =
            pbkdf2_hmac_array::<Sha256, 32>(login_info.password.as_bytes(), salt.as_bytes(), 4096);

        Credentials {
            username: login_info.username,
            password_hash: hex::encode(hashed_password),
            salt,
        }
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}
