extern crate redis;

mod models;

use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, MultipartOptions};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use crate::models::meeting::{MeetingSchema, MutationRoot, QueryRoot, Storage};

async fn index(schema: web::Data<MeetingSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = redis::Client::open("redis://redis/").expect("failed to open redis");

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::from(client))
        .finish();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(index)
                    .app_data(MultipartOptions::default().max_num_files(3)),
            )
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind("0.0.0.0:7070")?
    .run()
    .await
}
