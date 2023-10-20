use crate::models::role::Role;

use async_graphql::{Context, Object};
use sqlx::PgPool;

#[derive(Default)]
pub struct RoleQuery;

#[Object]
impl RoleQuery {
    async fn role(&self, ctx: &Context<'_>, id: i32) -> Option<Role> {
        let pool = ctx.data_unchecked::<PgPool>();
        Role::find_by_id(pool, id).await.ok().flatten()
    }

    async fn roles(&self, ctx: &Context<'_>) -> Vec<Role> {
        let pool = ctx.data_unchecked::<PgPool>();
        Role::find_all(pool).await.ok().unwrap_or_default()
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
    ) -> Option<Role> {
        let pool = ctx.data_unchecked::<PgPool>();
        Role::create(pool, &name, description.as_deref()).await.ok()
    }

    async fn update_role(
        &self,
        ctx: &Context<'_>,
        id: i32,
        name: String,
        description: Option<String>,
    ) -> Option<Role> {
        let pool = ctx.data_unchecked::<PgPool>();
        Role::update(pool, id, &name, description.as_deref())
            .await
            .ok()
    }

    async fn delete_role(&self, ctx: &Context<'_>, id: i32) -> bool {
        let pool = ctx.data_unchecked::<PgPool>();
        Role::delete(pool, id)
            .await
            .ok()
            .map(|rows| rows > 0)
            .unwrap_or(false)
    }
}
