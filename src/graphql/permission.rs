use async_graphql::{ComplexObject, Context, Object};
use sqlx::PgPool;

use crate::models::{permission::Permission, role::Role, role_permission::RolePermission};

#[ComplexObject]
impl Permission {
    async fn roles(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Role>> {
        let pool = ctx.data::<PgPool>()?;
        let role_ids = RolePermission::roles_by_permission(&pool, self.id).await?;

        let mut roles = Vec::new();

        for rid in role_ids {
            if let Some(role) = Role::find_by_id(&pool, rid).await? {
                roles.push(role);
            }
        }

        Ok(roles)
    }
}

#[derive(Default)]
pub struct PermissionQuery;

#[Object]
impl PermissionQuery {
    async fn permissions(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Permission>> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Permission::find_all(&pool).await?)
    }

    async fn permission_by_id(
        &self,
        ctx: &Context<'_>,
        role_id: i32,
    ) -> async_graphql::Result<Option<Permission>> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Permission::find_by_id(&pool, role_id).await?)
    }

    async fn roles_by_permission(
        &self,
        ctx: &Context<'_>,
        permission_id: i32,
    ) -> async_graphql::Result<Vec<Role>> {
        let pool = ctx.data::<PgPool>()?;
        let role_ids = RolePermission::roles_by_permission(&pool, permission_id).await?;

        let mut roles = Vec::new();
        for rid in role_ids {
            if let Some(role) = Role::find_by_id(&pool, rid).await? {
                roles.push(role);
            }
        }

        Ok(roles)
    }
}

#[derive(Default)]
pub struct PermissionMutation;

#[Object]
impl PermissionMutation {
    async fn create_permission(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: Option<String>,
    ) -> async_graphql::Result<Permission> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Permission::create(&pool, &name, description.as_deref()).await?)
    }

    async fn update_permission(
        &self,
        ctx: &Context<'_>,
        id: i32,
        name: String,
        description: Option<String>,
    ) -> async_graphql::Result<Permission> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Permission::update(&pool, id, &name, description.as_deref()).await?)
    }

    async fn delete_permission(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<u64> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Permission::delete(&pool, id).await?)
    }
}
