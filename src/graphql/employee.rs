use crate::models::employee::Employee;
use crate::models::role::Role;

use async_graphql::*;
use sqlx::PgPool;

#[ComplexObject]
impl Employee {
    async fn role(&self, ctx: &Context<'_>) -> Result<Option<Role>> {
        if let Some(role_id) = self.role_id {
            let pool = ctx.data::<PgPool>()?;
            Ok(Role::find_by_id(&pool, role_id).await?)
        } else {
            Ok(None)
        }
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
        role_id: Option<i32>,
        team_id: Option<i32>,
    ) -> Result<Employee> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Employee::create(&pool, &name, role_id, team_id).await?)
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
}
