mod cluster;
mod db;

#[macro_use]
extern crate lazy_static;

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{
    get, middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

mod schema2;

use crate::schema2::{create_schema, Schema};

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect(".env file not found");

    pretty_env_logger::init();

    // Create Juniper schema
    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql_playground)
            .service(graphql)
            // the graphiql UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
