use std::env;
use mongodb::{
    Client as MongoClient,
    options::{ClientOptions, StreamAddress, Credential},
};
use async_graphql::{
    Result as GQLResult,
    Error as GQLError
};

use crate::load_env;

// A helper function for implementations of the DbClient trait that gets a handle to a DB client from environment variables
// All errors are given in GraphQL format, seeing as this function will be called in resolver logic and conversion is annoying
pub fn get_client() -> GQLResult<MongoClient> {
    load_env().expect("Error getting environment variables!");
    // Get all the necessary configuration from environment variables
    let hostname = env::var("DB_HOSTNAME").expect("Environment variable 'DB_HOSTNAME' not present or invalid.");
    let port = env::var("DB_PORT")
        .expect("Environment variable 'DB_PORT' not present or invalid.")
        .parse::<u16>() // Ports are not going to be larger than a 16-bit integer
        .expect("Environment variable 'DB_PORT' should be a number, but it isn't.");
    let username = env::var("DB_USERNAME").expect("Environment variable 'DB_USERNAME' not present or invalid.");
    let password = env::var("DB_PASSWORD").expect("Environment variable 'DB_PASSWORD' not present or invalid.");
    let options =
        ClientOptions::builder()
            .hosts(vec![
                StreamAddress {
                    hostname,
                    port: Some(port),
                }
            ])
            .credential(
                Credential::builder()
                    .username(username)
                    .password(password)
                    .build()
            )
            .build();
    let client = MongoClient::with_options(options);
    match client {
        Ok(client) => Ok(client),
        // TODO revisit this error message
        Err(_) => Err(GQLError::new("Error connecting to database"))
    }
}

// The MongoDB crate handles pooling internally, so we don't have to worry about it here
// We just need a struct that exposes methods to get a client
// If extra pooling logic ever needs to be added, it can be done from here
pub struct DbPool {}
impl DbPool {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_client(&self) -> GQLResult<MongoClient> {
        // Check if we already have a client cached
        let client = get_client()?;

        Ok(client)
    }
}
