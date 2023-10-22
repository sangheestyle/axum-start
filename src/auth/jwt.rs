use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Header, Validation};
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID or username
    exp: i64,
}

impl Claims {
    pub fn new(sub: String) -> Self {
        Claims {
            sub,
            exp: (Utc::now() + Duration::hours(24)).timestamp(),
        }
    }
}

fn get_encoding_key() -> Result<EncodingKey, jsonwebtoken::errors::Error> {
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    Ok(EncodingKey::from_secret(secret.as_bytes()))
}

fn get_decoding_key() -> Result<DecodingKey, jsonwebtoken::errors::Error> {
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    Ok(DecodingKey::from_secret(secret.as_bytes()))
}

pub fn create_token(claims: Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let key = get_encoding_key()?;
    encode(&Header::default(), &claims, &key)
}

pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let key = get_decoding_key()?;
    decode::<Claims>(token, &key, &Validation::default()).map(|data| data.claims)
}
