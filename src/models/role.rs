use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use sqlx::{FromRow, PgPool};

#[derive(FromRow, SimpleObject, Debug)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Role {
    // Fetch a role by ID
    pub async fn find_by_id(pool: &PgPool, role_id: i32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Role,
            r#"
            SELECT * FROM roles WHERE id = $1
            "#,
            role_id
        )
        .fetch_optional(pool)
        .await
    }

    // Fetch all roles
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Role,
            r#"
            SELECT * FROM roles
            "#,
        )
        .fetch_all(pool)
        .await
    }

    // Insert a new Role
    pub async fn create(
        pool: &PgPool,
        name: &str,
        description: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Role,
            r#"
            INSERT INTO roles (name, description)
            VALUES ($1, $2)
            RETURNING *;
            "#,
            name,
            description
        )
        .fetch_one(pool)
        .await
    }

    // Update an existing Role by ID
    pub async fn update(
        pool: &PgPool,
        id: i32,
        name: &str,
        description: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Role,
            r#"
            UPDATE roles
            SET name = $2, description = $3, updated_at = NOW()
            WHERE id = $1
            RETURNING *;
            "#,
            id,
            name,
            description
        )
        .fetch_one(pool)
        .await
    }

    // Delete a Role by ID
    pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM roles WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected())
    }
}
