use crate::graphql::{MutationRoot, QueryRoot, SubscriptionRoot};

use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use sqlx::postgres::PgPool;
use std::error::Error;

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

pub async fn start(pool: PgPool) -> Result<(), Box<dyn Error>> {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(pool)
        .finish();

    let app = Router::new()
        .route(
            "/graphiql",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .route_service("/ws", GraphQLSubscription::new(schema));

    println!("GraphiQL: http://localhost:8000/graphiql");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
