// This binary runs a serverful setup with Actix, as opposed to a serverless approach (TODO)

use lib::{load_env, get_schema};

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
