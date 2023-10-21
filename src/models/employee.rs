use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use sqlx::{FromRow, PgPool};

use super::{role::Role, team::Team};

#[derive(FromRow, SimpleObject, Debug)]
#[graphql(complex)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub role_id: Option<i32>,
    pub team_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Employee {
    // Fetch an employee by ID
    pub async fn find_by_id(pool: &PgPool, emp_id: i32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Employee,
            r#"
            SELECT * FROM employees WHERE id = $1
            "#,
            emp_id
        )
        .fetch_optional(pool)
        .await
    }

    // Fetch all employees
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Employee,
            r#"
            SELECT * FROM employees
            "#,
        )
        .fetch_all(pool)
        .await
    }

    // Insert a new Employee
    pub async fn create(
        pool: &PgPool,
        name: &str,
        role_id: Option<i32>,
        team_id: Option<i32>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Employee,
            r#"
            INSERT INTO employees (name, role_id, team_id)
            VALUES ($1, $2, $3)
            RETURNING *;
            "#,
            name,
            role_id,
            team_id
        )
        .fetch_one(pool)
        .await
    }

    // Update an existing Employee by ID
    pub async fn update(
        pool: &PgPool,
        id: i32,
        name: &str,
        role_id: Option<i32>,
        team_id: Option<i32>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Employee,
            r#"
            UPDATE employees
            SET name = $2, role_id = $3, team_id = $4, updated_at = NOW()
            WHERE id = $1
            RETURNING *;
            "#,
            id,
            name,
            role_id,
            team_id
        )
        .fetch_one(pool)
        .await
    }

    // Delete an Employee by ID
    pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM employees WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected())
    }

    pub async fn get_team(&self, pool: &PgPool) -> Result<Option<Team>, sqlx::Error> {
        match self.team_id {
            Some(tid) => Team::find_by_id(pool, tid).await,
            None => Ok(None),
        }
    }

    pub async fn get_role(&self, pool: &PgPool) -> Result<Option<Role>, sqlx::Error> {
        match self.role_id {
            Some(rid) => Role::find_by_id(pool, rid).await,
            None => Ok(None),
        }
    }

    pub async fn assign_role(
        pool: &PgPool,
        employee_id: i32,
        role_id: i32,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Employee,
            r#"
            UPDATE employees
            SET role_id = $2
            WHERE id = $1
            RETURNING *;
            "#,
            employee_id,
            role_id
        )
        .fetch_one(pool)
        .await
    }
}
