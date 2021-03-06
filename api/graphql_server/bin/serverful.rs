#![forbid(unsafe_code)]
// This binary runs a serverful setup with Actix Web, as opposed to a serverless approach (TODO)
// Even so, this system does NOT support subscriptions so we maintain the separity in development that will be present in production

use async_graphql_actix_web::{Request, Response, WSSubscription};
use async_graphql::Schema;
use actix_web::{guard, web, App, HttpServer, HttpRequest, HttpResponse, Result as ActixResult};
use lib::{
    load_env,
    AppSchemaWithoutSubscriptions as AppSchema,
    get_schema_without_subscriptions as get_schema,
    routes::{
        graphiql
    }
};

const GRAPHIQL_ENDPOINT: &str = "/graphiql"; // For the graphical development playground
const GRAPHQL_ENDPOINT: &str = "/graphql";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env().expect("Error getting environment variables!");
    // We get the schema once and then use it for all queries
    // If this fails, we can't do anything at all
    let schema = get_schema().expect("Failed to fetch schema.");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource(GRAPHQL_ENDPOINT).guard(guard::Post()).to(graphql)) // POST endpoint for queries/mutations
            .service(web::resource(GRAPHQL_ENDPOINT)
                .guard(guard::Get())
                .guard(guard::Header("upgrade", "websocket"))
                .to(graphql_ws)
            ) // WebSocket endpoint for subscriptions
            .service(web::resource(GRAPHIQL_ENDPOINT).guard(guard::Get()).to(graphiql)) // GET endpoint for GraphiQL playground
    })
    .bind("0.0.0.0:7000")? // This stays the same, that port in the container will get forwarded to whatever's configured in `.ports.env`
    .run()
    .await
}

async fn graphql(
    schema: web::Data<AppSchema>,
    req: Request,
) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_ws(
    schema: web::Data<AppSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> ActixResult<HttpResponse> {
    WSSubscription::start(Schema::clone(&schema), &req, payload)
}
