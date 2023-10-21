use super::client::Client;
use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use sqlx::{FromRow, PgPool};

#[derive(FromRow, SimpleObject, Debug)]
#[graphql(complex)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Team {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(Team, "SELECT * FROM teams")
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_id(pool: &PgPool, team_id: i32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(Team, "SELECT * FROM teams WHERE id = $1", team_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(pool: &PgPool, name: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Team,
            "INSERT INTO teams (name) VALUES ($1) RETURNING *",
            name
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update(pool: &PgPool, id: i32, name: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Team,
            "UPDATE teams SET name = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
            id,
            name
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
        sqlx::query!("DELETE FROM teams WHERE id = $1", id)
            .execute(pool)
            .await
            .map(|result| result.rows_affected())
    }

    pub async fn get_clients(&self, pool: &PgPool) -> Result<Vec<Client>, sqlx::Error> {
        sqlx::query_as!(
            Client,
            r#"
            SELECT * FROM clients WHERE team_id = $1
            "#,
            self.id
        )
        .fetch_all(pool)
        .await
    }
}
