use std::time::Duration;

use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    results::InsertOneResult,
    Client, IndexModel,
};

use mongodb::error::Result as MongoResult;

use crate::Credentials;

#[derive(Clone, Debug)]
pub struct MongoDatabase {
    client: Client,
    database: mongodb::Database,
}

impl MongoDatabase {
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

    pub async fn write_credentials(
        &self,
        credentials: &Credentials,
    ) -> MongoResult<InsertOneResult> {
        let collection = self.database.collection::<Credentials>("credentials");

        collection.insert_one(credentials, None).await
    }

    pub async fn fetch_credentials(&self, username: String) -> Option<Credentials> {
        let collection = self.database.collection::<Credentials>("credentials");

        collection
            .find_one(doc! { "username": username }, None)
            .await
            .unwrap()
    }

    pub async fn attempt_register(&self, credentials: Credentials) -> Option<String> {
        let collection = self.database.collection::<Credentials>("credentials");

        let mut session = self.client.start_session(None).await.ok()?;

        let existing = collection
            .find_one_with_session(
                doc! { "username": credentials.username() },
                None,
                &mut session,
            )
            .await
            .ok()?;

        if existing.is_some() {
            return None;
        }

        collection
            .insert_one_with_session(credentials, None, &mut session)
            .await
            .ok()?;

        Some("session token".to_string())
    }
}
