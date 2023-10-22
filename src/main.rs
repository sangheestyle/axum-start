mod auth;
mod graphql;
mod models;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    http::{header::AUTHORIZATION, HeaderName, Method},
    middleware,
    response::{self, Html, IntoResponse},
    routing::{get, post, post_service},
    Router, Server,
};
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

use auth::jwt::jwt_middleware;
use auth::login::{login, logout, signup};
use graphql::schema::create_schema;

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

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    // redis_client: redis::Client,
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

    let schema = create_schema(pool.clone(), redis_client);
    let state = AppState { pool: pool.clone() };

    let app = Router::new()
        .route("/graphql", post_service(GraphQL::new(schema.clone())))
        .route_layer(middleware::from_fn(jwt_middleware))
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/graphiql", get(graphql_playground))
        .route_service("/ws", GraphQLSubscription::new(schema))
        .route("/health", get(health_check))
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
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
