mod graphql;
mod models;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    http::{header::AUTHORIZATION, HeaderName, Method},
    response::{self, Html, IntoResponse},
    routing::{get, post_service},
    Router, Server,
};
use graphql::schema::create_schema;
use serde_json::json;
use std::{iter::once, time::Duration};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing_subscriber::fmt;

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/ws"),
    ))
}

async fn health_check() -> impl IntoResponse {
    response::Json(json!({
        "status": "Healthy"
    }))
}

#[tokio::main]
async fn main() {
    fmt::init();
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool.");
    let redis_client = redis::Client::open(redis_url).expect("Failed to create Redis client.");

    let schema = create_schema(pool, redis_client.clone());

    let app = Router::new()
        .route("/graphiql", get(graphql_playground))
        .route("/graphql", post_service(GraphQL::new(schema.clone())))
        .route_service("/ws", GraphQLSubscription::new(schema))
        .route("/health", get(health_check))
        .layer(
            ServiceBuilder::new()
                // Mark the `Authorization` request header as sensitive so it doesn't show in logs
                .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
                // High level logging of requests and responses
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(Duration::from_secs(10)))
                .layer(CompressionLayer::new())
                .layer(PropagateHeaderLayer::new(HeaderName::from_static(
                    "x-request-id",
                )))
                .layer(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST])
                        .allow_origin(Any),
                ),
        );

    println!("GraphiQL: http://localhost:8000/graphiql");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
