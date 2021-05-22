#![forbid(unsafe_code)]
// This binary runs a serverful setup with Actix Web, as opposed to a serverless approach (TODO)
// Even so, this system does NOT support subscriptions so we maintain the separity in development that will be present in production

use async_graphql_actix_web::{Request, Response, WSSubscription};
use async_graphql::{Schema, http::{playground_source, GraphQLPlaygroundConfig}};
use actix_web::{guard, web, App, HttpServer, HttpRequest, HttpResponse, Result as ActixResult};
use lib::{
    load_env,
    AppSchemaWithoutSubscriptions as AppSchema,
    get_schema_without_subscriptions as get_schema
};

const GRAPHIQL_ENDPOINT: &str = "/graphiql"; // For the graphical development playground
const GRAPHQL_ENDPOINT: &str = "/graphql";

// TODO formed schema caching logic

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env().expect("Error getting environment variables!");
    // We get the schema once and then use it for all queries
    // TODO convert string errors into IO errors
    let schema = get_schema().map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

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

// TODO only compile this in development
// The endpoint for the development graphical playground
async fn graphiql() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new(GRAPHQL_ENDPOINT).subscription_endpoint(GRAPHQL_ENDPOINT),
        )))
}
