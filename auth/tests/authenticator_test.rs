use auth::{error::AuthError, AuthenticateResult, LoginInfo, SessionToken};

#[derive(Clone)]
struct Authenticator {
    inner: auth::db::Authenticator,
    dropped: bool,
}

impl Authenticator {
    fn new(auth: auth::db::Authenticator) -> Self {
        Self {
            inner: auth,
            dropped: false,
        }
    }

    async fn drop_database(&self) {
        let client = self.inner.get_client().await;
        let database = client.database("auth");
        database.drop(None).await.expect("Failed to drop database");
    }

    async fn register(&self, info: LoginInfo) -> Result<SessionToken, AuthError> {
        self.inner.register(info).await
    }

    async fn login(&self, info: LoginInfo) -> Result<SessionToken, AuthError> {
        self.inner.login(info).await
    }

    async fn logout(&self, token: SessionToken) -> Result<(), AuthError> {
        self.inner.logout(token).await
    }

    async fn authenticate(&self, token: &str) -> Result<AuthenticateResult, AuthError> {
        self.inner.authenticate(token).await
    }
}

impl Drop for Authenticator {
    fn drop(&mut self) {
        if !self.dropped {
            let mut this = self.clone();
            std::mem::swap(&mut this, self);
            this.dropped = true;
            tokio::spawn(async move { this.drop_database().await });
        }
    }
}

async fn get_authenticator() -> Authenticator {
    let authenticator = Authenticator::new(
        auth::db::Authenticator::new("mongodb://localhost:27017".to_string(), "auth".to_string())
            .await
            .expect("Failed to connect to MongoDB"),
    );

    println!("Created authenticator");
    authenticator.drop_database().await;

    authenticator
}

macro_rules! assert_not_error {
    ($result:expr) => {
        if $result.is_err() {
            panic!("{}", format!("{:?}", $result.unwrap_err()));
        }
    };
}

#[tokio::test]
async fn test_register_succeeds() {
    let auth = get_authenticator().await;
    let result = auth.register(LoginInfo::new("username", "password")).await;

    assert_not_error!(result);
}

#[tokio::test]
async fn test_register_fails_if_username_is_taken() {
    let auth = get_authenticator().await;
    let result = auth.register(LoginInfo::new("username", "password")).await;
    assert_not_error!(result);

    let result = auth
        .register(LoginInfo::new("username", "another password"))
        .await;

    if result.is_ok() {
        panic!("Expected to fail registering a username that is already taken");
    }

    assert_eq!(
        result.unwrap_err(),
        AuthError::UsernameTaken("username".to_string())
    );
}

#[tokio::test]
async fn test_register_returns_session_token() {
    let auth = get_authenticator().await;
    let result = auth.register(LoginInfo::new("username", "password")).await;
    assert_not_error!(result);

    let token = result.unwrap();
    assert_eq!(token.username(), "username");
}

#[tokio::test]
async fn test_register_session_token_can_auth() {
    let auth = get_authenticator().await;
    let result = auth.register(LoginInfo::new("username", "password")).await;
    assert_not_error!(result);

    let token = result.unwrap();

    let result = auth.authenticate(token.token()).await;
    assert_not_error!(result);
}

#[tokio::test]
async fn test_invalid_session_token_fails_auth() {
    let auth = get_authenticator().await;
    let result = auth.register(LoginInfo::new("username", "password")).await;
    assert_not_error!(result);

    let token = result.unwrap();
    let wrong_token = SessionToken::from_parts(token.username(), "token");

    let result = auth.authenticate(wrong_token.token()).await;

    if result.is_ok() {
        panic!("Expected to fail authenticating with an invalid token");
    }

    assert_eq!(result.unwrap_err(), AuthError::AuthenticationError);
}

#[tokio::test]
async fn test_login_fails_before_register() {
    let auth = get_authenticator().await;

    let info = LoginInfo::new("username", "password");
    let result = auth.login(info.clone()).await;
    assert!(result.is_err(), "Login should fail before registration");
}

#[tokio::test]
async fn test_can_login_after_register() {
    let auth = get_authenticator().await;
    let info = LoginInfo::new("username", "password");
    let result = auth.register(info.clone()).await;
    assert_not_error!(result);

    let result = auth.login(info.clone()).await;
    assert_not_error!(result);
}

#[tokio::test]
async fn test_login_wrong_password() {
    let auth = get_authenticator().await;
    let result = auth.register(LoginInfo::new("username", "password")).await;
    assert_not_error!(result);

    let wrong_info = LoginInfo::new("username", "wrong password");
    let result = auth.login(wrong_info.clone()).await;
    assert!(result.is_err(), "Login should fail with wrong password");
}

#[tokio::test]
async fn test_login_returns_session_token() {
    let auth = get_authenticator().await;
    let info = LoginInfo::new("username", "password");

    assert_not_error!(auth.register(info.clone()).await);

    let result = auth.login(info.clone()).await;
    assert_not_error!(result);

    let token = result.unwrap();
    assert_eq!(token.username(), "username");
}

#[tokio::test]
async fn test_login_session_token_can_auth() {
    let auth = get_authenticator().await;
    let info = LoginInfo::new("username", "password");

    assert_not_error!(auth.register(info.clone()).await);

    let result = auth.login(info.clone()).await;
    assert_not_error!(result);

    let token = result.unwrap();

    let result = auth.authenticate(token.token()).await;
    assert_not_error!(result);
}

#[tokio::test]
async fn test_logout() {
    let auth = get_authenticator().await;
    let info = LoginInfo::new("username", "password");

    assert_not_error!(auth.register(info.clone()).await);

    let result = auth.login(info.clone()).await;
    assert_not_error!(result);

    let token = result.unwrap();
    assert_not_error!(auth.logout(token.clone()).await);

    let result = auth.authenticate(token.token()).await;
    assert!(result.is_err(), "Login should fail after logout");
}
