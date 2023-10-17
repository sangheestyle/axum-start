use crate::graphql::{MutationRoot, QueryRoot, SubscriptionRoot};

use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::{get, post_service},
    Router, Server,
};
use sqlx::postgres::PgPool;
use std::error::Error;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub async fn start(pool: PgPool) -> Result<(), Box<dyn Error>> {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(pool)
        .finish();

    let app = Router::new()
        .route("/graphql", post_service(GraphQL::new(schema)))
        .route("/graphiql", get(graphiql));

    println!("GraphiQL: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
