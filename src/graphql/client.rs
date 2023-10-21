use async_graphql::{ComplexObject, Context, Object, Result};
use sqlx::PgPool;

use crate::models::{client::Client, team::Team};

#[ComplexObject]
impl Client {
    async fn team(&self, ctx: &Context<'_>) -> Result<Option<Team>> {
        match self.team_id {
            Some(team_id) => {
                let pool = ctx.data::<PgPool>()?;
                Ok(Team::find_by_id(&pool, team_id).await?)
            }
            None => Ok(None),
        }
    }
}

#[derive(Default)]
pub struct ClientQuery;

#[Object]
impl ClientQuery {
    async fn clients(&self, ctx: &Context<'_>) -> Result<Vec<Client>> {
        let pool = ctx.data::<PgPool>()?;
        Client::find_all(&pool).await.map_err(|e| e.into())
    }

    async fn client_by_id(&self, ctx: &Context<'_>, client_id: i32) -> Result<Option<Client>> {
        let pool = ctx.data::<PgPool>()?;
        Client::find_by_id(&pool, client_id)
            .await
            .map_err(|e| e.into())
    }
}

#[derive(Default)]
pub struct ClientMutation;

#[Object]
impl ClientMutation {
    async fn create_client(
        &self,
        ctx: &Context<'_>,
        name: String,
        team_id: Option<i32>,
    ) -> Result<Client> {
        let pool = ctx.data::<PgPool>()?;
        Client::create(&pool, name, team_id)
            .await
            .map_err(|e| e.into())
    }

    async fn delete_client(&self, ctx: &Context<'_>, client_id: i32) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;
        Client::delete_by_id(&pool, client_id)
            .await
            .map(|num| num > 0)
            .map_err(|e| e.into())
    }

    async fn update_client(
        &self,
        ctx: &Context<'_>,
        client_id: i32,
        name: String,
        team_id: Option<i32>,
    ) -> Result<Client> {
        let pool = ctx.data::<PgPool>()?;
        Client::update(&pool, client_id, name, team_id)
            .await
            .map_err(|e| e.into())
    }

    async fn assign_client_to_team(
        &self,
        ctx: &Context<'_>,
        client_id: i32,
        team_id: i32,
    ) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;
        Client::assign_to_team(&pool, client_id, team_id)
            .await
            .map_err(|e| e.into())
    }

    async fn remove_client_from_team(&self, ctx: &Context<'_>, client_id: i32) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;
        Client::remove_from_team(&pool, client_id)
            .await
            .map_err(|e| e.into())
    }
}
