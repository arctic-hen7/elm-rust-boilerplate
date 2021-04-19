use tokio::stream::StreamExt;
use serde::{Serialize, Deserialize};
use async_graphql::{
    SimpleObject as GQLSimpleObject,
    Object as GQLObject,
    Result as GQLResult,
    InputObject as GQLInputObject,
    Error as GQLError
};
use mongodb::{
    bson::{
        doc,
        oid::ObjectId
    },
    Client as MongoClient,
    Collection
};
use crate::graphql::get_client_from_ctx;

#[derive(Serialize, Deserialize, Debug, GQLSimpleObject)]
pub struct User {
    // The `_id` field must be skipped entirely if you want to query with it (otherwise deserialization breaks the type acceptance)
    #[serde(skip)]
    pub oid: String,
    pub username: String,
    pub full_name: Option<String>,
    pub password: String
}
// TODO add a macro to make all fields of a struct optional
#[derive(Serialize, Deserialize, Debug, GQLInputObject)]
pub struct PartialUser {
    #[serde(skip)]
    pub oid: Option<String>,
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub password: Option<String>
}
#[derive(Serialize, Deserialize, Debug, GQLInputObject)]
pub struct UserInput {
    pub username: String,
    pub full_name: Option<String>,
    pub password: String
}

fn get_users(client: MongoClient) -> Collection<User> {
    client.database("test").collection_with_type::<User>("users")
}

// Register Query methods
#[derive(Default)]
pub struct Query {}
#[GQLObject]
impl Query {
    async fn users(&self, ctx: &async_graphql::Context<'_>, username: String) -> GQLResult<Vec<User>> {
        let users = get_users(get_client_from_ctx(ctx)?);

        let mut cursor = users.find(doc! {
            "username": username
        }, None).await?;
        let mut res: Vec<User> = Vec::new();
        while let Some(user) = cursor.next().await {
            res.push(user?);
        }
        Ok(res)
    }
}

// Register Mutation methods
#[derive(Default)]
pub struct Mutation {}
#[GQLObject]
impl Mutation {
    async fn add_user(&self, ctx: &async_graphql::Context<'_>, new_user: UserInput) -> GQLResult<User> {
        let users = get_users(get_client_from_ctx(ctx)?);
        let users_input: Collection<UserInput> = users.clone_with_type();

        let inserted_id = users_input.insert_one(new_user, None).await?.inserted_id.as_object_id().unwrap().to_hex();
        let inserted = users.find_one(doc! {
            "_id": ObjectId::with_string(&inserted_id).unwrap() // The string came from an ObjectId, we know more than the compiler
        }, None).await?;

        match inserted {
            Some(inserted) => Ok(User {
                oid: inserted_id,
                ..inserted
            }),
            None => Err(GQLError::new("Couldn't find inserted field"))
        }
    }
}
