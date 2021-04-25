// We merge all the component schemas together here, so this code will need to be regularly updated in early development

use crate::schemas::{
    users::{
        Query as UsersQuery,
        Mutation as UsersMutation,
        Subscription as UsersSubscription
    }
};

#[derive(MergedObject, Default)]
pub struct QueryRoot(BaseQuery, UsersQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(UsersMutation);

#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(UsersSubscription);

// GRAPHQL CODE

use async_graphql::{
    MergedObject,
    MergedSubscription,
    Object as GQLObject,
    Result as GQLResult,
    Schema,
};
use mongodb::Client as MongoClient;

use crate::db::DbPool;

// We make an instance of the database client accessible to all GraphQL resolvers through context
pub struct Context {
    pub pool: DbPool, // This needs to be public so that schema files can use it
}

// A helper function to get a client from the given context object
pub fn get_client_from_ctx(raw_ctx: &async_graphql::Context<'_>) -> GQLResult<MongoClient> {
    // Extract our context from the broader `async_graphql` context
    let ctx = raw_ctx.data::<Context>()?;
    let client = ctx.pool.get_client()?;

    Ok(client)
}

// The base query type unrelated to any particular logic
#[derive(Default)]
struct BaseQuery;
#[GQLObject]
impl BaseQuery {
    // All APIs should implement this method for best practices so clients know what the hell they're doing
    async fn api_version(&self) -> String {
        // TODO use an environment variable to get the API version
        "v0.1.0".to_string()
    }
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub fn get_schema() -> AppSchema {
    Schema::build(QueryRoot::default(), MutationRoot::default(), SubscriptionRoot::default())
        .data(Context {
            pool: DbPool::new()
        })
        .finish()
}
