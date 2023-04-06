use std::time::Duration;

use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    Client, ClientSession, Database, IndexModel,
};

use crate::{error::AuthError, Credentials, LoginInfo, SessionToken};

#[derive(Clone, Debug)]
pub struct Authenticator {
    client: Client,
    database: Database,
}

impl Authenticator {
    pub async fn new(url: String, db_name: String) -> anyhow::Result<Self> {
        let mut client_options = ClientOptions::parse(url).await?;
        client_options.app_name = Some("auth".to_string());

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

    pub async fn attempt_register(&self, info: LoginInfo) -> Result<SessionToken, AuthError> {
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
            return Err(AuthError::UsernameTaken(credentials.username().clone()));
        }

        credentials_collection
            .insert_one_with_session(credentials, None, &mut session)
            .await?;

        self.create_and_store_session_token(info.username, &mut session)
            .await
    }

    async fn create_and_store_session_token(
        &self,
        username: String,
        session: &mut ClientSession,
    ) -> Result<SessionToken, AuthError> {
        let session_token = SessionToken::new(username);
        let session_token_collection = self.database.collection::<SessionToken>("sessions");
        session_token_collection
            .insert_one_with_session(session_token.clone(), None, session)
            .await?;

        Ok(session_token)
    }

    pub async fn login(&self, info: LoginInfo) -> Result<SessionToken, AuthError> {
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
                None => return Err(AuthError::UserNotFound(info.username)),
            }
        };

        if credentials.matches(&info) {
            self.create_and_store_session_token(info.username, &mut session)
                .await
        } else {
            Err(AuthError::InvalidPassword)
        }
    }

    pub async fn authenticate(&self, session_token: SessionToken) -> Result<(), AuthError> {
        let mut session = self.client.start_session(None).await?;
        let session_token_collection = self.database.collection::<SessionToken>("sessions");

        let session_token_option = session_token_collection
            .find_one_with_session(
                doc! { "username": session_token.username(), "token": session_token.token() },
                None,
                &mut session,
            )
            .await?;

        match session_token_option {
            Some(_) => Ok(()),
            None => Err(AuthError::AuthenticationError),
        }
    }

    pub async fn logout(&self, session_token: SessionToken) -> Result<(), AuthError> {
        let mut session = self.client.start_session(None).await?;
        let session_token_collection = self.database.collection::<SessionToken>("sessions");

        session_token_collection
            .delete_one_with_session(
                doc! { "username": session_token.username(), "token": session_token.token() },
                None,
                &mut session,
            )
            .await?;

        Ok(())
    }
}
