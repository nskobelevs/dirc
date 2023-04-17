use pbkdf2::pbkdf2_hmac_array;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

pub mod db;
pub mod error;

#[derive(Deserialize)]
pub struct UserExistsParams {
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct AuthenticateResult {
    pub username: String,
}

impl From<String> for AuthenticateResult {
    fn from(username: String) -> Self {
        AuthenticateResult { username }
    }
}

/// A struct that contains the username and password
#[derive(Clone, Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

impl LoginInfo {
    /// Creates a new LoginInfo struct
    pub fn new(username: &str, password: &str) -> Self {
        LoginInfo {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

/// A struct that contains the username and a session token
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionToken {
    username: String,
    token: String,
}

impl SessionToken {
    /// Creates a new random SessionToken given a username
    pub fn new(username: String) -> Self {
        let token = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>();

        SessionToken { username, token }
    }

    /// Returns the username of the SessionToken
    pub fn username(&self) -> &String {
        &self.username
    }

    /// Returns the token of the SessionToken
    pub fn token(&self) -> &String {
        &self.token
    }

    /// Creates a new SessionToken from a username and a token
    pub fn from_parts(username: &str, token: &str) -> Self {
        SessionToken {
            username: username.to_string(),
            token: token.to_string(),
        }
    }
}

/// A struct that contains the username, password hash and salt of a user
#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password_hash: String,
    salt: String,
}

impl Credentials {
    /// Creates a new Credentials struct given a LoginInfo
    ///
    /// Will generate a random salt and hash the password with the salt
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

    /// Returns the username of the Credentials
    pub fn username(&self) -> &String {
        &self.username
    }

    /// Returns true if the given password matches the password of the Credentials
    pub fn matches(&self, login_info: &LoginInfo) -> bool {
        let hashed_password = Credentials::create_hash(&login_info.password, &self.salt);

        self.password_hash == hex::encode(hashed_password)
    }
}
