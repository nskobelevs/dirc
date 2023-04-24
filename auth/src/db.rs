use std::time::Duration;

use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    Client, ClientSession, Database, IndexModel,
};

use core_rs::{error::ServiceError, ProfilePicture};

use crate::{AuthenticateResult, Credentials, LoginInfo, SessionToken};

const DEFAULT_PROFILE_PICTURE_URL: &str =
    "https://upload.wikimedia.org/wikipedia/commons/2/2c/Default_pfp.svg";

/// Authenticator is the main struct for the authentication service handing authentication actions using a MongoDB database.
#[derive(Clone, Debug)]
pub struct Authenticator {
    client: Client,
    database: Database,
}

impl Authenticator {
    pub async fn get_client(&self) -> Client {
        self.client.clone()
    }

    /// Creates a new Authenticator instance given a mongodb url and database name
    ///
    /// # Errors
    /// Construction will fail if a database error occurs
    pub async fn new(url: String, db_name: String) -> anyhow::Result<Self> {
        let mut client_options = ClientOptions::parse(url).await?;
        client_options.app_name = Some("auth".to_string());
        client_options.connect_timeout = Some(Duration::from_secs(3));
        client_options.server_selection_timeout = Some(Duration::from_secs(10));

        let client = Client::with_options(client_options)?;
        let database = client.database(db_name.as_str());

        let credentials_options = IndexOptions::builder().unique(true).build();
        let credentials_model = IndexModel::builder()
            .keys(doc! {"username": 1})
            .options(credentials_options)
            .build();

        database
            .collection::<Credentials>("credentials")
            .create_index(credentials_model, None)
            .await?;

        let session_options = IndexOptions::builder()
            .expire_after(Duration::from_secs(2592000))
            .build();
        let session_model = IndexModel::builder()
            .keys(doc! {"token": 1})
            .options(session_options)
            .build();

        database
            .collection::<Credentials>("sessions")
            .create_index(session_model, None)
            .await?;

        Ok(Self { client, database })
    }

    /// Registers a new user with the given username and password
    ///
    /// # Errors
    /// `ServiceError::DatabaseError` if a database error occurs
    /// `ServiceError::UsernameTaken` if the username is already taken
    pub async fn register(&self, info: LoginInfo) -> Result<SessionToken, ServiceError> {
        let credentials = Credentials::new(&info);

        let credentials_collection = self.database.collection::<Credentials>("credentials");

        let mut session = self.client.start_session(None).await?;

        let existing = credentials_collection
            .find_one_with_session(
                doc! { "username": credentials.username() },
                None,
                &mut session,
            )
            .await?;

        if existing.is_some() {
            return Err(ServiceError::UsernameTaken(credentials.username().clone()));
        }

        credentials_collection
            .insert_one_with_session(credentials, None, &mut session)
            .await?;

        let token_res = self
            .create_and_store_session_token(info.username.clone(), &mut session)
            .await;

        match token_res {
            Ok(session_token) => {
                let username = info.username.clone();

                if username.starts_with("test") {
                    return Ok(session_token);
                }

                let client = awc::Client::default();

                let pfp: ProfilePicture = DEFAULT_PROFILE_PICTURE_URL.to_string().into();

                let mut response = client
                    .put(format!("http://users:8080/{}/info", username))
                    .send_json(&pfp)
                    .await
                    .expect("Failed to make request to auth/authenticate");

                if response.status() == 200 {
                    Ok(session_token)
                } else {
                    println!("Error: {:?}", response.body().await);
                    Err(ServiceError::AuthenticationError)
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Creates a new session token, stores it in the database and returns it
    ///
    /// # Errors
    /// `ServiceError::DatabaseError` if a database error occurs
    async fn create_and_store_session_token(
        &self,
        username: String,
        session: &mut ClientSession,
    ) -> Result<SessionToken, ServiceError> {
        let session_token = SessionToken::new(username);
        let session_token_collection = self.database.collection::<SessionToken>("sessions");
        session_token_collection
            .insert_one_with_session(session_token.clone(), None, session)
            .await?;

        Ok(session_token)
    }

    /// Logs in a user with the given username and password
    ///
    /// # Errors
    /// `ServiceError::DatabaseError` if a database error occurs
    /// `ServiceError::UserNotFound` if the user does not exist
    /// `ServiceError::InvalidPassword` if the password is incorrect
    pub async fn login(&self, info: LoginInfo) -> Result<SessionToken, ServiceError> {
        let mut session = self.client.start_session(None).await?;

        let credentials_collection = self.database.collection::<Credentials>("credentials");

        let credentials = {
            let credentials_option = credentials_collection
                .find_one_with_session(
                    doc! { "username": info.username.clone() },
                    None,
                    &mut session,
                )
                .await?;

            match credentials_option {
                Some(credentials) => credentials,
                None => return Err(ServiceError::UserNotFound(info.username)),
            }
        };

        if credentials.matches(&info) {
            self.create_and_store_session_token(info.username, &mut session)
                .await
        } else {
            Err(ServiceError::InvalidPassword)
        }
    }

    /// Authenticates a session token
    ///
    /// # Errors
    /// `ServiceError::DatabaseError` if a database error occurs
    /// `ServiceError::AuthenticationError` if the session token is invalid
    pub async fn authenticate(
        &self,
        session_token: &str,
    ) -> Result<AuthenticateResult, ServiceError> {
        let mut session = self.client.start_session(None).await?;
        let session_token_collection = self.database.collection::<SessionToken>("sessions");

        let session_token_option = session_token_collection
            .find_one_with_session(doc! { "token": session_token }, None, &mut session)
            .await?;

        match session_token_option {
            Some(session_token_object) => Ok(session_token_object.username().clone().into()),
            None => Err(ServiceError::AuthenticationError),
        }
    }

    /// Logs out a user with the given session token
    ///
    /// # Errors
    /// `ServiceError::DatabaseError` if a database error occurs
    pub async fn logout(&self, session_token: &String) -> Result<(), ServiceError> {
        let mut session = self.client.start_session(None).await?;
        let session_token_collection = self.database.collection::<SessionToken>("sessions");

        session_token_collection
            .delete_one_with_session(doc! { "token": session_token }, None, &mut session)
            .await?;

        Ok(())
    }

    /// Checks whether a user with a given username exists
    ///
    /// # Errors
    /// `ServiceError::DatabaseError` if a database error occurs
    pub async fn user_exists(&self, username: String) -> Result<bool, ServiceError> {
        let mut session = self.client.start_session(None).await?;
        let credentials_collection = self.database.collection::<Credentials>("credentials");

        let credentials_option = credentials_collection
            .find_one_with_session(doc! { "username": username.clone() }, None, &mut session)
            .await?;

        Ok(credentials_option.is_some())
    }
}
