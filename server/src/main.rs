extern crate redis;
mod models;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::http::{Method};
use axum::response::{self, IntoResponse};
use axum::routing::{get};
use axum::{extract::Extension, Router, Server};
use tower_http::cors::{CorsLayer, Origin, Any, AnyOr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::models::meeting::{MeetingSchema, MutationRoot, QueryRoot, SubscriptionRoot, Storage};
use serde::Deserialize;


async fn graphql_handler(schema: Extension<MeetingSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

#[derive(Deserialize, Debug)]
struct EnvConfig {
  redis_url: String,
  allow_origin: Option<String>,
}

#[tokio::main]
async fn main() {
    let config = match envy::from_env::<EnvConfig>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error)
     };

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let client = redis::Client::open(config.redis_url)
        .expect("failed to open redis");
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::from(client))
        .finish();
    let allow_origin: AnyOr<Origin>= match config.allow_origin {
        Some(url) => Origin::exact(url.parse().unwrap()).into(),
        None => Any.into(),
    };
    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/ws", GraphQLSubscription::new(schema.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin(allow_origin)
                .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS, Method::HEAD]),
        )
        .layer(Extension(schema));
    println!("Playground: http://localhost:7070");
    Server::bind(&"0.0.0.0:7070".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
