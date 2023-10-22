use crate::auth::hashing::{generate_salt, hash_password, verify_password};
use crate::auth::jwt::create_token;
use crate::auth::jwt::Claims;
use crate::models::employee::Employee;
use async_graphql::{Context, Error, Object, Result};

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> Result<String> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let employee = Employee::find_by_username(&pool, &email).await?;

        match employee {
            Some(emp) => {
                if verify_password(&password, &emp.password_hash, &emp.salt) {
                    let claims = Claims::new(emp.id.to_string());
                    let token = create_token(claims)?;
                    Ok(token)
                } else {
                    Err(Error::new("Incorrect password."))
                }
            }
            None => Err(Error::new("Employee not found.")),
        }
    }

    async fn logout(&self) -> Result<&str> {
        // In a real-world scenario, you might want to invalidate the token or perform other logout related tasks.
        Ok("Logged out successfully.")
    }

    async fn signup(
        &self,
        ctx: &Context<'_>,
        name: String,
        email: String,
        password: String,
    ) -> Result<String> {
        let pool = ctx.data::<sqlx::PgPool>()?;

        // Check if a user with the given email already exists
        let existing_employee = Employee::find_by_username(&pool, &email).await?;
        if existing_employee.is_some() {
            return Err(Error::new("A user with this email already exists."));
        }

        let salt = generate_salt();
        let hashed_password = hash_password(&password, &salt);

        // Create a new employee
        let _ = Employee::create(
            &pool,
            &name,
            &email,
            &hashed_password,
            &salt,
            None, // role_id
            None, // team_id
        )
        .await?;

        Ok("User registered successfully.".to_string())
    }
}
