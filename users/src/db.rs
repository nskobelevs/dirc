use std::time::Duration;

use core_rs::error::ServiceError;
use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    Client, Database, IndexModel,
};

use crate::{ProfilePicture, User};

#[derive(Clone, Debug)]
pub struct Users {
    client: Client,
    database: Database,
}

impl Users {
    pub async fn get_client(&self) -> Client {
        self.client.clone()
    }

    /// Creates a new Authenticator instance given a mongodb url and database name
    ///
    /// # Errors
    /// Construction will fail if a database error occurs
    pub async fn new(url: String, db_name: String) -> anyhow::Result<Self> {
        let mut client_options = ClientOptions::parse(url).await?;
        client_options.app_name = Some("users".to_string());
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
            .collection::<User>("users")
            .create_index(credentials_model, None)
            .await?;

        Ok(Self { client, database })
    }

    /// Checks if a user with the given username exists
    ///
    /// # Errors
    /// `AuthError::DatabaseError` if a database error occurs
    pub async fn exists(&self, username: String) -> Result<bool, ServiceError> {
        let mut session = self.client.start_session(None).await?;

        let user_collection = self.database.collection::<User>("users");

        let credentials_option = user_collection
            .find_one_with_session(doc! { "username": username.clone() }, None, &mut session)
            .await?;

        Ok(credentials_option.is_some())
    }

    /// Gets the user info for the given username
    ///
    /// # Errors
    /// `AuthError::DatabaseError` if a database error occurs
    /// `AuthError::UserNotFound` if the user does not exist
    pub async fn info(&self, username: String) -> Result<User, ServiceError> {
        let mut session = self.client.start_session(None).await?;

        let user_collection = self.database.collection::<User>("users");

        let user_option = user_collection
            .find_one_with_session(doc! { "username": username.clone() }, None, &mut session)
            .await?;

        if let Some(user) = user_option {
            Ok(user)
        } else {
            Err(ServiceError::UserNotFound(username))
        }
    }

    /// Puts the user info for the given username
    ///
    /// # Errors
    /// `AuthError::DatabaseError` if a database error occurs
    pub async fn save_info(
        &self,
        username: String,
        profile_picture: ProfilePicture,
    ) -> Result<(), ServiceError> {
        let mut session = self.client.start_session(None).await?;

        let user_collection = self.database.collection::<User>("users");

        let user_option = user_collection
            .find_one_with_session(doc! { "username": username.clone() }, None, &mut session)
            .await?;

        match user_option {
            Some(mut user) => {
                user.profile_picture = profile_picture.profile_picture.clone();

                user_collection
                    .replace_one_with_session(
                        doc! { "username": username.clone() },
                        user,
                        None,
                        &mut session,
                    )
                    .await?;
            }
            None => {
                let user = User {
                    username: username.clone(),
                    profile_picture: profile_picture.profile_picture.clone(),
                };

                user_collection
                    .insert_one_with_session(user, None, &mut session)
                    .await?;
            }
        };

        Ok(())
    }
}
