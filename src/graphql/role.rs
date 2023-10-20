use async_graphql::{Context, Object};
use sqlx::PgPool;

use crate::models::role::Role;

#[derive(Default)]
pub struct RoleQuery;

#[Object]
impl RoleQuery {
    async fn roles(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Role>> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Role::find_all(&pool).await?)
    }

    async fn role_by_id(
        &self,
        ctx: &Context<'_>,
        role_id: i32,
    ) -> async_graphql::Result<Option<Role>> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Role::find_by_id(&pool, role_id).await?)
    }
}

#[derive(Default)]
pub struct RoleMutation;

#[Object]
impl RoleMutation {
    async fn create_role(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: Option<String>,
    ) -> async_graphql::Result<Role> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Role::create(&pool, &name, description.as_deref()).await?)
    }

    async fn update_role(
        &self,
        ctx: &Context<'_>,
        id: i32,
        name: String,
        description: Option<String>,
    ) -> async_graphql::Result<Role> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Role::update(&pool, id, &name, description.as_deref()).await?)
    }

    async fn delete_role(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<u64> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Role::delete(&pool, id).await?)
    }
}
