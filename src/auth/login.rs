use crate::auth::hashing::verify_password;
use crate::models::employee::Employee;
use axum::{extract::Extension, response, BoxError, Json};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn login(
    Extension(pool): Extension<PgPool>,
    Json(body): axum::extract::Json<LoginRequest>,
) -> Result<response::Json<serde_json::Value>, BoxError> {
    let user_result = Employee::find_by_username(&pool, &body.username).await;

    match user_result {
        Ok(Some(user)) => {
            if verify_password(&body.password, &user.password_hash, &user.salt) {
                let claims = crate::auth::jwt::Claims::new(user.id.to_string());
                let token = match crate::auth::jwt::create_token(claims) {
                    Ok(t) => t,
                    Err(e) => {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Token creation error: {}", e),
                        )))
                    }
                };
                Ok(response::Json(json!({"token": token})))
            } else {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Invalid password",
                )))
            }
        }
        Ok(None) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "User not found",
        ))),
        Err(e) => Err(Box::new(std::io::Error::new(
            // Handle database error here
            std::io::ErrorKind::Other,
            format!("Database error: {}", e),
        ))),
    }
}
