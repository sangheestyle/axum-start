use std::error::Error;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_axum::*;
use axum::{
    response::{self, IntoResponse},
    routing::{get, post_service},
    Router, Server,
};

struct Query;

#[Object]
impl Query {
    async fn howdy(&self) -> &'static str {
        "partner"
    }
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();
    let app = Router::new()
        .route("/graphql", post_service(GraphQL::new(schema)))
        .route("/graphiql", get(graphiql));

    println!("GraphiQL: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
