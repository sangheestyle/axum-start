use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;

use crate::models::employee::Employee as EmployeeModel;

pub struct EmployeeQuery;

#[Object]
impl EmployeeQuery {
    async fn employee_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<Option<EmployeeModel>> {
        let pool = ctx.data::<PgPool>()?;
        let id: i32 = id.parse()?;
        Ok(EmployeeModel::find_by_id(&pool, id).await?)
    }

    async fn all_employees(&self, ctx: &Context<'_>) -> Result<Vec<EmployeeModel>> {
        let pool = ctx.data::<PgPool>()?;
        Ok(EmployeeModel::find_all(&pool).await?)
    }
}

pub struct EmployeeMutation;

#[Object]
impl EmployeeMutation {
    async fn create_employee(
        &self,
        ctx: &Context<'_>,
        name: String,
        role_id: Option<i32>,
        team_id: Option<i32>,
    ) -> Result<EmployeeModel> {
        let pool = ctx.data::<PgPool>()?;
        Ok(EmployeeModel::create(&pool, &name, role_id, team_id).await?)
    }

    async fn update_employee(
        &self,
        ctx: &Context<'_>,
        id: ID,
        name: String,
        role_id: Option<i32>,
        team_id: Option<i32>,
    ) -> Result<EmployeeModel> {
        let pool = ctx.data::<PgPool>()?;
        let id: i32 = id.parse()?;
        Ok(EmployeeModel::update(&pool, id, &name, role_id, team_id).await?)
    }

    async fn delete_employee(&self, ctx: &Context<'_>, id: ID) -> Result<i32> {
        let pool = ctx.data::<PgPool>()?;
        let id: i32 = id.parse()?;
        EmployeeModel::delete(&pool, id).await?;
        Ok(id)
    }
}
