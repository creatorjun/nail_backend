// src/infrastructure/jwt.rs
use anyhow::{anyhow, Result};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub provider: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn issue_access_token(user_id: Uuid, role: &str, provider: &str) -> Result<String> {
    let secret = env::var("JWT_SECRET")?;
    let expiry_secs: i64 = env::var("JWT_ACCESS_EXPIRY_SECONDS")
        .unwrap_or_else(|_| "900".into())
        .parse()?;

    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        provider: provider.to_string(),
        iat: now,
        exp: now + expiry_secs,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| anyhow!(e))
}

pub fn verify_access_token(token: &str) -> Result<Claims> {
    let secret = env::var("JWT_SECRET")?;
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| anyhow!(e))?;
    Ok(token_data.claims)
}
