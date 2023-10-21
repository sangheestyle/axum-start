use crate::graphql::client::{ClientMutation, ClientQuery};
use crate::graphql::employee::{EmployeeMutation, EmployeeQuery, EmployeeSubscription};
use crate::graphql::permission::{PermissionMutation, PermissionQuery};
use crate::graphql::role::{RoleMutation, RoleQuery};
use crate::graphql::team::{TeamMutation, TeamQuery};

use async_graphql::{MergedObject, Schema};

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

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmployeeSubscription>;

pub fn create_schema(pool: sqlx::PgPool, redis_client: redis::Client) -> AppSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmployeeSubscription,
    )
    .data(pool)
    .data(redis_client)
    .finish()
}
