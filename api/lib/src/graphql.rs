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

use std::sync::Mutex;
use async_graphql::{
    MergedObject,
    MergedSubscription,
    Object as GQLObject,
    Result as GQLResult,
    Schema,
    EmptySubscription
};
use mongodb::Client as MongoClient;
use tokio::stream::Stream;

use crate::db::DbPool;
use crate::pubsub::{PubSub, Publisher};

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

// A helper function to subscribe to events sent to the subscriptions server on a particular channel
// This returns a pre-created stream which you should manipulate if necessary (e.g. to serialise data)
// ONLY USE THIS IN SUBSCRIPTIONS! It will only run on the serverful system (stateful)
pub fn get_stream_for_channel_from_ctx(channel: &str, raw_ctx: &async_graphql::Context<'_>) -> impl Stream<Item = String> {
    // Extract our context from the broader `async_graphql` context
    let pubsub_mutex = raw_ctx.data::<Mutex<PubSub>>().unwrap(); // If we can't find data that should very clearly be in context, this should panic!
    let mut pubsub = pubsub_mutex.lock().unwrap(); // FIXME
    // Return a stream on the given channel
    pubsub.subscribe(channel)
}

// The base query type unrelated to any particular logic
// This needs to be public because it's used directly by the subscriptions server
#[derive(Default)]
pub struct BaseQuery;
#[GQLObject]
impl BaseQuery {
    // All APIs should implement this method for best practices so clients know what the hell they're doing
    async fn api_version(&self) -> String {
        // TODO use an environment variable to get the API version
        "v0.1.0".to_string()
    }
}

// This mutation type is utilised by the subscriptions server to allow the publishing of data
// We pass around the PubSub state internally to that GraphQL system (see get_schema_for_subscriptions)
#[derive(Default)]
pub struct PublishMutation;
#[GQLObject]
impl PublishMutation {
    // We accept string data because this is a highly generic type that serialises in the subscriptions handler
    // That may seem to subvert some of the purpose of GraphQL, but this resolver is to be INTERNALLY ONLY!
    // That provides a system-level data integrity guarantee, as only full mutations will call this, and through a PubSub abstraction
    // There should be very little reason for users to implement it themselves, but this type could easily be extended with custom logic
    // TODO authenticate that messages here have actually come from the rest of the system
    async fn publish(&self, raw_ctx: &async_graphql::Context<'_>, channel: String, data: String) -> GQLResult<bool> {
        // We store the PubSub instance as a Mutex because we need it sent/synced between threads as a mutable
        let pubsub_mutex = raw_ctx.data::<Mutex<PubSub>>()?;
        let mut pubsub = pubsub_mutex.lock()?;

        pubsub.publish(&channel, data);
        Ok(true)
    }
}

// Serverless functions cannnot handle subscriptions, so we separate the schema here
pub type AppSchemaWithoutSubscriptions = Schema<QueryRoot, MutationRoot, EmptySubscription>;
// We need to be able to work out the API version on the subscriptions server, so we still provide the basic queries
pub type AppSchemaForSubscriptions = Schema<BaseQuery, PublishMutation, SubscriptionRoot>;

pub fn get_schema_without_subscriptions() -> Result<AppSchemaWithoutSubscriptions, String> {
    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(Context {
            pool: DbPool::new()
        })
        .data(
            Publisher::new(None, None, None)?
        ) // We add a publisher that can send data to the subscriptions server (you can provide a hostname and port here if you want)
        .finish();

    Ok(schema)
}
pub fn get_schema_for_subscriptions() -> AppSchemaForSubscriptions {
    Schema::build(BaseQuery, PublishMutation, SubscriptionRoot::default())
        .data(Context {
            pool: DbPool::new()
        })
        .data(
            Mutex::new(PubSub::new())
        ) // We add a PubSub instance to internally manage state in the serverful subscriptions system
        .finish()
}
