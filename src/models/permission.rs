use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use sqlx::{FromRow, PgPool};

#[derive(FromRow, SimpleObject, Debug)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Permission {
    pub async fn find_by_id(
        pool: &PgPool,
        permission_id: i32,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Permission,
            r#"
            SELECT * FROM permissions WHERE id = $1
            "#,
            permission_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Permission,
            r#"
            SELECT * FROM permissions
            "#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        name: &str,
        description: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Permission,
            r#"
            INSERT INTO permissions (name, description)
            VALUES ($1, $2)
            RETURNING *;
            "#,
            name,
            description
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update(
        pool: &PgPool,
        id: i32,
        name: &str,
        description: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Permission,
            r#"
            UPDATE permissions
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

    pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM permissions WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected())
    }

    // pub async fn roles(&self, pool: &PgPool) -> Result<Vec<Role>, sqlx::Error> {
    //     sqlx::query_as!(
    //         Role,
    //         r#"
    //         SELECT roles.*
    //         FROM roles
    //         JOIN role_permissions ON role_permissions.role_id = roles.id
    //         WHERE role_permissions.permission_id = $1
    //         "#,
    //         self.id
    //     )
    //     .fetch_all(pool)
    //     .await
    // }
}
