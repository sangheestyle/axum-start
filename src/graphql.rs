use crate::todo::*;
use async_graphql::{Context, Enum, FieldResult, Object, SimpleObject, Subscription, ID};
use futures::Stream;
use sqlx::postgres::PgPool;

#[Object]
impl Todo {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn body(&self) -> &str {
        &self.body
    }

    async fn complete(&self) -> &bool {
        &self.complete
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn todos(&self, ctx: &Context<'_>) -> FieldResult<Vec<Todo>> {
        let pool = ctx.data::<PgPool>()?;
        let items = Todo::list(&pool).await?;

        Ok(items)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_todo(&self, ctx: &Context<'_>, body: String) -> FieldResult<Todo> {
        let pool = ctx.data::<PgPool>()?;
        let item = Todo::insert(&pool, &body).await?;

        // SimpleBroker::publish(TodoChanged {
        //     mutation_type: MutationType::Created,
        //     id: item.clone().id.into(),
        //     item: Some(item.clone()),
        // });

        Ok(item)
    }

    async fn delete_todo(&self, ctx: &Context<'_>, id: ID) -> FieldResult<bool> {
        let pool = ctx.data::<PgPool>()?;
        let id = id.parse::<String>()?;

        Todo::delete(&pool, &id).await?;

        // SimpleBroker::publish(TodoChanged {
        //     mutation_type: MutationType::Deleted,
        //     id: id.into(),
        //     item: None,
        // });

        Ok(true)
    }

    async fn update_todo(
        &self,
        ctx: &Context<'_>,
        id: ID,
        body: String,
        complete: bool,
    ) -> FieldResult<Option<Todo>> {
        let pool = ctx.data::<PgPool>()?;
        let id = id.parse::<String>()?;

        let item = Todo::update(&pool, &id, &body, complete).await?;

        // SimpleBroker::publish(TodoChanged {
        //     mutation_type: MutationType::Updated,
        //     id: id.into(),
        //     item: item.clone(),
        // });

        Ok(item)
    }

    async fn toggle_complete(&self, ctx: &Context<'_>, id: ID) -> FieldResult<Option<Todo>> {
        let pool = ctx.data::<PgPool>()?;
        let id = id.parse::<String>()?;

        let item = Todo::toggle_complete(&pool, &id).await?;

        // SimpleBroker::publish(TodoChanged {
        //     mutation_type: MutationType::Updated,
        //     id: id.into(),
        //     item: item.clone(),
        // });

        Ok(item)
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
enum MutationType {
    Created,
    Updated,
    Deleted,
}

#[derive(Clone, SimpleObject)]
struct TodoChanged {
    mutation_type: MutationType,
    id: ID,
    item: Option<Todo>,
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn events1(&self) -> impl Stream<Item = i32> {
        futures_util::stream::iter(0..10)
    }
    // async fn todos(&self, mutation_type: Option<MutationType>) -> impl Stream<Item = TodoChanged> {
    //     SimpleBroker::<TodoChanged>::subscribe().filter(move |event| {
    //         if let Some(mutation_type) = mutation_type {
    //             event.mutation_type == mutation_type
    //         } else {
    //             true
    //         }
    //     })
    // }
}
