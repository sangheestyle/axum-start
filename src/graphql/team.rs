use crate::models::{client::Client, team::Team};
use async_graphql::{ComplexObject, Context, Object};
use sqlx::PgPool;

#[ComplexObject]
impl Team {
    async fn clients(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Client>> {
        let pool = ctx.data::<PgPool>()?;
        Ok(self.get_clients(&pool).await?)
    }
}

#[derive(Default)]
pub struct TeamQuery;

#[Object]
impl TeamQuery {
    async fn teams(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Team>> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Team::find_all(&pool).await?)
    }

    async fn team_by_id(
        &self,
        ctx: &Context<'_>,
        team_id: i32,
    ) -> async_graphql::Result<Option<Team>> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Team::find_by_id(&pool, team_id).await?)
    }
}

#[derive(Default)]
pub struct TeamMutation;

#[Object]
impl TeamMutation {
    async fn create_team(&self, ctx: &Context<'_>, name: String) -> async_graphql::Result<Team> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Team::create(&pool, &name).await?)
    }

    async fn update_team(
        &self,
        ctx: &Context<'_>,
        id: i32,
        name: String,
    ) -> async_graphql::Result<Team> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Team::update(&pool, id, &name).await?)
    }

    async fn delete_team(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<u64> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Team::delete(&pool, id).await?)
    }
}
