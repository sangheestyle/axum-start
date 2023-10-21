use crate::graphql::client::{ClientMutation, ClientQuery};
use crate::graphql::employee::{EmployeeMutation, EmployeeQuery};
use crate::graphql::permission::{PermissionMutation, PermissionQuery};
use crate::graphql::role::{RoleMutation, RoleQuery};
use crate::graphql::team::{TeamMutation, TeamQuery};

use async_graphql::{EmptySubscription, MergedObject, Schema};

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    EmployeeQuery,
    RoleQuery,
    PermissionQuery,
    TeamQuery,
    ClientQuery,
);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    EmployeeMutation,
    RoleMutation,
    PermissionMutation,
    TeamMutation,
    ClientMutation,
);

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
