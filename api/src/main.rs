mod load_env;
mod db;
mod schema;

use crate::schema::{Client, DbClient, User};
use crate::load_env::load_env;

#[tokio::main]
async fn main() {
    load_env().expect("Error getting environment variables!");

    let new_user = User {
        username: "jane".to_string(),
        full_name: Some("Jane Doe".to_string()),
        password: "password".to_string()
    };

    let client = Client::new().unwrap();
    client.add_user(new_user).await.unwrap();
}
