use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use sqlx::Error;
use sqlx::{FromRow, PgPool};

#[derive(FromRow, Debug, SimpleObject)]
#[graphql(complex)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub team_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Client {
    pub async fn create(
        pool: &PgPool,
        name: String,
        team_id: Option<i32>,
    ) -> Result<Client, Error> {
        sqlx::query_as!(
            Client,
            r#"
            INSERT INTO clients (name, team_id)
            VALUES ($1, $2)
            RETURNING *
            "#,
            name,
            team_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Client>, Error> {
        sqlx::query_as!(
            Client,
            r#"
            SELECT * FROM clients
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, client_id: i32) -> Result<Option<Client>, Error> {
        sqlx::query_as!(
            Client,
            r#"
            SELECT * FROM clients
            WHERE id = $1
            "#,
            client_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn delete_by_id(pool: &PgPool, client_id: i32) -> Result<u64, Error> {
        sqlx::query!(
            r#"
            DELETE FROM clients
            WHERE id = $1
            "#,
            client_id
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected())
    }

    pub async fn update(
        pool: &PgPool,
        client_id: i32,
        name: String,
        team_id: Option<i32>,
    ) -> Result<Client, Error> {
        sqlx::query_as!(
            Client,
            r#"
            UPDATE clients
            SET name = $2, team_id = $3
            WHERE id = $1
            RETURNING *
            "#,
            client_id,
            name,
            team_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn assign_to_team(
        pool: &PgPool,
        client_id: i32,
        team_id: i32,
    ) -> Result<bool, Error> {
        let rows_modified = sqlx::query!(
            r#"
            UPDATE clients
            SET team_id = $1
            WHERE id = $2
            "#,
            team_id,
            client_id
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(rows_modified > 0)
    }

    pub async fn remove_from_team(pool: &PgPool, client_id: i32) -> Result<bool, Error> {
        let rows_modified = sqlx::query!(
            r#"
            UPDATE clients
            SET team_id = NULL
            WHERE id = $1
            "#,
            client_id
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(rows_modified > 0)
    }
}
