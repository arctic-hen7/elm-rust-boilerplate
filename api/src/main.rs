mod load_env;
mod db;
mod schemas;
mod graphql;

use crate::graphql::get_schema;
use crate::load_env::load_env;

#[tokio::main]
async fn main() {
    load_env().expect("Error getting environment variables!");

    // let query = "
    //     mutation {
    //         addUser(newUser: {
    //             username: \"john\",
    //             fullName: \"John Doe\",
    //             password: \"password\"
    //         }) {
    //             oid
    //             username
    //             fullName
    //         }
    //     }
    // ";
    let query = "
        query {
            users(username: \"jane\") {
                oid
                username
                fullName
            }
        }
    ";
    let schema = get_schema();
    let res = schema.execute(query).await;

    println!("{:#?}", res)
}
