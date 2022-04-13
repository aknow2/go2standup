extern crate redis;

mod models;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::http::Method;
use axum::response::{self, IntoResponse};
use axum::routing::get;
use axum::{extract::Extension, Router, Server};
use tower_http::cors::{CorsLayer, Any};
use crate::models::meeting::{MeetingSchema, MutationRoot, QueryRoot, SubscriptionRoot, Storage};


async fn graphql_handler(schema: Extension<MeetingSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://redis/").expect("failed to open redis");

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::from(client))
        .finish();
    
    let app = Router::new()
    .route("/", get(graphql_playground).post(graphql_handler))
    .route("/ws", GraphQLSubscription::new(schema.clone()))
    .layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS, Method::HEAD]),
    )
    .layer(Extension(schema));

    println!("Playground: http://localhost:7070");

    Server::bind(&"0.0.0.0:7070".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
