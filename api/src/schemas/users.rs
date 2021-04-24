use tokio::stream::StreamExt;
use serde::{Serialize, Deserialize};
use async_graphql::{
    SimpleObject as GQLSimpleObject,
    Object as GQLObject,
    Result as GQLResult,
    InputObject as GQLInputObject,
    Error as GQLError,
};
use mongodb::{
    bson::doc,
    Client as MongoClient,
    Collection
};
use crate::graphql::get_client_from_ctx;
use crate::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, GQLSimpleObject)]
pub struct User {
    // We need to use `id` because otherwise we can't access the field properly with Rust
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub full_name: Option<String>,
    pub password: String
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

        let insertion_res = users_input.insert_one(new_user, None).await?;
        let inserted = users.find_one(ObjectId::find_clause_from_insertion_res(insertion_res)?, None).await?;

        let insert_find_err = GQLError::new("Couldn't find inserted field");

        inserted.ok_or(
            insert_find_err
        )
    }
}
