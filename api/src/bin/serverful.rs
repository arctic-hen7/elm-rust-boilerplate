// This binary runs a serverful setup with Actix, as opposed to a serverless approach (TODO)

use async_graphql_actix_web::{Request, Response};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use actix_web::{guard, web, App, HttpServer, HttpResponse, Result as ActixResult};
use lib::{load_env, get_schema, AppSchema};

const GRAPHIQL_ENDPOINT: &str = "/graphiql"; // For the graphical development playground
const GRAPHQL_ENDPOINT: &str = "/graphql";

// TODO formed schema caching logic

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env().expect("Error getting environment variables!");
    // We get the schema once and then use it for all queries
     let schema = get_schema();

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource(GRAPHQL_ENDPOINT).guard(guard::Post()).to(graphql))
            .service(web::resource(GRAPHIQL_ENDPOINT).guard(guard::Get()).to(graphiql))
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

// TODO only compile this in development
// The endpoint for the development graphical playground
async fn graphiql() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new(GRAPHQL_ENDPOINT)// .subscription_endpoint(GRAPHIQL_ENDPOINT),
        )))
}
