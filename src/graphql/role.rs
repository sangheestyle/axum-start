use crate::models::{permission::Permission, role::Role, role_permission::RolePermission};

use async_graphql::{ComplexObject, Context, Object, Result};
use sqlx::PgPool;

#[ComplexObject]
impl Role {
    async fn permissions(&self, ctx: &Context<'_>) -> Result<Vec<Permission>> {
        // TODO: optimize query
        let pool = ctx.data::<PgPool>()?;
        let permission_ids = RolePermission::permissions_by_role(&pool, self.id).await?;

        let mut permissions = Vec::new();

        for pid in permission_ids {
            if let Some(permission) = Permission::find_by_id(&pool, pid).await? {
                permissions.push(permission);
            }
        }

        Ok(permissions)
    }
}

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

    async fn add_permissions_to_role(
        &self,
        ctx: &Context<'_>,
        role_id: i32,
        permission_ids: Vec<i32>,
    ) -> async_graphql::Result<u64> {
        let pool = ctx.data::<PgPool>()?;
        Ok(RolePermission::assign_permissions_to_role(&pool, role_id, &permission_ids).await?)
    }

    async fn remove_permissions_from_role(
        &self,
        ctx: &Context<'_>,
        role_id: i32,
        permission_ids: Vec<i32>,
    ) -> async_graphql::Result<u64> {
        let pool = ctx.data::<PgPool>()?;
        Ok(RolePermission::remove_permissions_from_role(&pool, role_id, &permission_ids).await?)
    }
}
