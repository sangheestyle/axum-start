use crate::auth::hashing::{generate_salt, hash_password, verify_password};
use crate::auth::jwt::create_token;
use crate::models::employee::Employee;
use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use serde::Deserialize;

use super::jwt::Claims;

#[derive(Deserialize)]
pub struct SignupInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Form(data): Form<SignupInput>,
) -> impl IntoResponse {
    // Check if a user with the given email already exists
    let existing_employee = Employee::find_by_username(&state.pool, &data.email)
        .await
        .unwrap_or(None);
    if existing_employee.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            "A user with this email already exists.",
        );
    }

    let salt = generate_salt();
    let hashed_password = hash_password(&data.password, &salt);

    // Create a new employee
    let _ = Employee::create(
        &state.pool,
        &data.name,
        &data.email,
        &hashed_password,
        &salt,
        None, // role_id
        None, // team_id
    )
    .await;

    (StatusCode::OK, "User registered successfully.")
}

pub async fn login(
    State(state): State<AppState>,
    Form(data): Form<LoginInput>,
) -> impl IntoResponse {
    let employee = Employee::find_by_username(&state.pool, &data.email)
        .await
        .unwrap_or(None);

    match employee {
        Some(emp) => {
            if verify_password(&data.password, &emp.password_hash, &emp.salt) {
                let claims = Claims::new(emp.id.to_string());
                match create_token(claims) {
                    Ok(token) => (StatusCode::OK, token),
                    Err(_) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to create token.".to_string(),
                    ),
                }
            } else {
                (StatusCode::UNAUTHORIZED, "Incorrect password.".to_string())
            }
        }
        None => (StatusCode::NOT_FOUND, "Employee not found.".to_string()),
    }
}

pub async fn logout() -> impl IntoResponse {
    // As before, this is a simple stub. In a real-world scenario,
    // you might want to invalidate the token or perform other logout related tasks.
    (StatusCode::OK, "Logged out successfully.")
}
