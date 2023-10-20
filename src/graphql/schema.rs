use crate::graphql::employee::{EmployeeMutation, EmployeeQuery};
use crate::graphql::role::{RoleMutation, RoleQuery};

use async_graphql::{EmptySubscription, MergedObject, Schema};

#[derive(MergedObject, Default)]
pub struct QueryRoot(EmployeeQuery, RoleQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(EmployeeMutation, RoleMutation);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(pool: sqlx::PgPool) -> AppSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(pool)
    .finish()
}
