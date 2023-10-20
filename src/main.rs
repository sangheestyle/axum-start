mod graphql;
mod models;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    response::{self, IntoResponse},
    routing::{get, post_service},
    Router, Server,
};

use graphql::schema::create_schema;

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
    let database_url = "postgres://sanghee@localhost:5432/axum";
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let schema = create_schema(pool);

    let app = Router::new()
        .route("/graphiql", get(graphiql))
        .route("/graphql", post_service(GraphQL::new(schema.clone())))
        .route_service("/ws", GraphQLSubscription::new(schema));

    println!("GraphiQL: http://localhost:8000/graphiql");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
