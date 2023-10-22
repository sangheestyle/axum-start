use crate::models::{employee::Employee, role::Role, team::Team};

use async_graphql::*;
use futures_util::Stream;
use redis::AsyncCommands;
use sqlx::PgPool;

use futures_util::StreamExt;

#[ComplexObject]
impl Employee {
    async fn team(&self, ctx: &Context<'_>) -> Result<Option<Team>> {
        let pool = ctx.data::<PgPool>()?;
        self.get_team(&pool).await.map_err(Into::into)
    }

    async fn role(&self, ctx: &Context<'_>) -> Result<Option<Role>> {
        let pool = ctx.data::<PgPool>()?;
        self.get_role(&pool).await.map_err(Into::into)
    }
}

#[derive(Default)]
pub struct EmployeeQuery;

#[Object]
impl EmployeeQuery {
    async fn employees(&self, ctx: &Context<'_>) -> Result<Vec<Employee>> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Employee::find_all(&pool).await?)
    }

    async fn employee_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Employee>> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Employee::find_by_id(&pool, id).await?)
    }
}

#[derive(Default)]
pub struct EmployeeMutation;

#[Object]
impl EmployeeMutation {
    async fn create_employee(
        &self,
        ctx: &Context<'_>,
        name: String,
        username: String,
        password_hash: String,
        salt: String,
        role_id: Option<i32>,
        team_id: Option<i32>,
    ) -> Result<Employee> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Employee::create(
            &pool,
            &name,
            &username,
            &password_hash,
            &salt,
            role_id,
            team_id,
        )
        .await?)
    }

    async fn update_employee(
        &self,
        ctx: &Context<'_>,
        id: i32,
        name: String,
        role_id: Option<i32>,
        team_id: Option<i32>,
    ) -> Result<Employee> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Employee::update(&pool, id, &name, role_id, team_id).await?)
    }

    async fn delete_employee(&self, ctx: &Context<'_>, id: i32) -> Result<u64> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Employee::delete(&pool, id).await?)
    }

    async fn assign_role_to_employee(
        &self,
        ctx: &Context<'_>,
        employee_id: i32,
        role_id: i32,
    ) -> async_graphql::Result<Employee> {
        let pool = ctx.data::<PgPool>()?;
        let result = Employee::assign_role(&pool, employee_id, role_id).await?;

        // Redis PubSub
        let redis_client: &redis::Client = ctx.data()?;
        let mut conn = redis_client.get_async_connection().await?;
        conn.publish(
            "assign_role_to_employee",
            format!("Role {} assigned to Employee {}", role_id, employee_id),
        )
        .await?;

        Ok(result)
    }

    async fn remove_role_from_employee(
        &self,
        ctx: &Context<'_>,
        employee_id: i32,
    ) -> async_graphql::Result<bool> {
        let pool = ctx.data::<PgPool>()?;
        Employee::remove_role(&pool, employee_id)
            .await
            .map_err(|e| async_graphql::Error::from(e))?;
        Ok(true)
    }

    async fn assign_employee_to_team(
        &self,
        ctx: &Context<'_>,
        employee_id: i32,
        team_id: i32,
    ) -> Result<Employee> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Employee::assign_to_team(&pool, employee_id, team_id).await?)
    }

    async fn remove_employee_from_team(&self, ctx: &Context<'_>, employee_id: i32) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;
        Employee::remove_from_team(&pool, employee_id).await?;
        Ok(true)
    }
}

#[derive(Default)]
pub struct EmployeeSubscription;

#[Subscription]
impl EmployeeSubscription {
    async fn role_assigned_to_employee(&self, ctx: &Context<'_>) -> impl Stream<Item = String> {
        let redis_client: &redis::Client = ctx.data().unwrap();
        let conn = redis_client.get_async_connection().await.unwrap();
        let mut pubsub = conn.into_pubsub();
        pubsub.subscribe("assign_role_to_employee").await.unwrap();
        pubsub.into_on_message().map(|msg| {
            let payload: String = msg.get_payload().unwrap();
            payload
        })
    }
}
