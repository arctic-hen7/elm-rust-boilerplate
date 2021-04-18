// We take the universal DB logic here and extend it for specific schemas
pub use crate::db::{DbClient, get_client};

use serde::{Serialize, Deserialize};
use mongodb::{
    Client as MongoClient,
    Collection,
    error::Error as MongoError,
    bson::doc
};

pub struct Client {
    client: MongoClient
}
impl DbClient for Client {
    fn new() -> Result<Self, MongoError> {
        let client = get_client()?;

        Ok(
            Self { client }
        )
    }
}

/**
 * Definitions of structs for collections and the methods for them.
 */

// Users
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub full_name: Option<String>,
    pub password: String
}
impl Client {
    fn get_users(&self) -> Collection<User> {
        let collection: Collection<User> = self.client.database("test").collection_with_type("users");
        collection
    }
    pub async fn add_user(&self, user: User) -> Result<(), MongoError> {
        let users = self.get_users();
        users.insert_one(user, None).await?;

        Ok(())
    }
}
