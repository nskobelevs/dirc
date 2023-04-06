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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionToken {
    username: String,
    token: String,
}

impl SessionToken {
    pub fn new(username: String) -> Self {
        let token = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>();

        SessionToken { username, token }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password_hash: String,
    salt: String,
}

impl Credentials {
    pub fn new(login_info: &LoginInfo) -> Self {
        let salt = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>();

        let hashed_password = Credentials::create_hash(&login_info.password, &salt);

        Credentials {
            username: login_info.username.clone(),
            password_hash: hex::encode(hashed_password),
            salt,
        }
    }

    fn create_hash(password: &String, salt: &String) -> String {
        let hashed_password =
            pbkdf2_hmac_array::<Sha256, 32>(password.as_bytes(), salt.as_bytes(), 4096);

        hex::encode(hashed_password)
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn matches(&self, login_info: &LoginInfo) -> bool {
        let hashed_password = Credentials::create_hash(&login_info.password, &self.salt);

        self.password_hash == hex::encode(hashed_password)
    }
}
