use crate::graphql::employee::{EmployeeMutation, EmployeeQuery};
use async_graphql::{EmptySubscription, Schema};

pub type AppSchema = Schema<EmployeeQuery, EmployeeMutation, EmptySubscription>;

pub fn create_schema(pool: sqlx::PgPool) -> AppSchema {
    Schema::build(EmployeeQuery, EmployeeMutation, EmptySubscription)
        .data(pool)
        .finish()
}
