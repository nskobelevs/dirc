use std::time::Duration;

use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    Client, Database, IndexModel,
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
            .unique(true)
            .expire_after(Duration::from_secs(2592000))
            .build();
        let session_model = IndexModel::builder()
            .keys(doc! {"username": 1})
            .options(session_options)
            .build();

        database
            .collection::<Credentials>("sessions")
            .create_index(session_model, None)
            .await?;

        Ok(Self { client, database })
    }

    pub async fn attempt_register(&self, info: LoginInfo) -> Result<SessionToken, AuthError> {
        let credentials = Credentials::new(info);

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
            .await
            .expect("Failed to insert user");

        let session_token = SessionToken::default();
        let session_token_collection = self.database.collection::<SessionToken>("sessions");
        session_token_collection
            .insert_one_with_session(session_token.clone(), None, &mut session)
            .await?;

        Ok(session_token)
    }
}
