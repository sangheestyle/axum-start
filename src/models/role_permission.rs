use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use sqlx::{FromRow, PgPool};

#[derive(FromRow, SimpleObject, Debug)]
pub struct RolePermission {
    pub role_id: i32,
    pub permission_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl RolePermission {
    // Fetch permissions by role ID
    pub async fn permissions_by_role(pool: &PgPool, role_id: i32) -> Result<Vec<i32>, sqlx::Error> {
        sqlx::query!(
            r#"
            SELECT permission_id FROM role_permissions WHERE role_id = $1
            "#,
            role_id
        )
        .fetch_all(pool)
        .await
        .map(|rows| rows.into_iter().map(|row| row.permission_id).collect())
    }

    // Fetch roles by permission ID
    pub async fn roles_by_permission(
        pool: &PgPool,
        permission_id: i32,
    ) -> Result<Vec<i32>, sqlx::Error> {
        sqlx::query!(
            r#"
            SELECT role_id FROM role_permissions WHERE permission_id = $1
            "#,
            permission_id
        )
        .fetch_all(pool)
        .await
        .map(|rows| rows.into_iter().map(|row| row.role_id).collect())
    }

    pub async fn assign_permissions_to_role(
        pool: &PgPool,
        role_id: i32,
        permission_ids: &[i32],
    ) -> Result<u64, sqlx::Error> {
        let mut total_affected = 0u64;
        for &pid in permission_ids {
            let affected = sqlx::query!(
                r#"
                INSERT INTO role_permissions (role_id, permission_id)
                VALUES ($1, $2)
                ON CONFLICT (role_id, permission_id) DO NOTHING
                "#,
                role_id,
                pid
            )
            .execute(pool)
            .await?
            .rows_affected();

            total_affected += affected;
        }
        Ok(total_affected)
    }
}
