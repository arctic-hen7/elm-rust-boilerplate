use std::env;
use mongodb::{
    Client,
    options::{ClientOptions, StreamAddress, Credential},
    error::Error as MongoError,
};

// A helper function for implementations of the DbClient trait that gets a handle to a DB client from environment variables
pub fn get_client() -> Result<Client, MongoError> {
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
    let client = Client::with_options(options)?;

    Ok(client)
}

pub trait DbClient {
    fn new() -> Result<Self, MongoError> where Self: std::marker::Sized;
}
