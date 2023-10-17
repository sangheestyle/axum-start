mod graphql;
mod result;
mod todo;
mod web;

use dotenv::dotenv;
use result::Result;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://sanghee@localhost:5432/axum")
        .await?;

    web::start(pool).await.unwrap();

    Ok(())
}
