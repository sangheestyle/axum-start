mod graphql;
mod models;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    http::{header::AUTHORIZATION, HeaderName, Method},
    response::{self, IntoResponse},
    routing::{get, post_service},
    Router, Server,
};
use graphql::schema::create_schema;
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

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    fmt::init();

    let database_url = "postgres://sanghee@localhost:5432/axum";
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let schema = create_schema(pool);

    let app = Router::new()
        .route("/graphiql", get(graphiql))
        .route("/graphql", post_service(GraphQL::new(schema.clone())))
        .route_service("/ws", GraphQLSubscription::new(schema))
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
