// src/application/auth_service.rs
use crate::domain::user::User;
use crate::infrastructure::crypto::{decrypt_social_id, encrypt_social_id};
use crate::infrastructure::jwt::issue_access_token;
use crate::infrastructure::social::SocialProfile;
use anyhow::{anyhow, Result};
use chrono::{Duration, Utc};
use rand::RngCore;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub user: User,
}

fn hash_provider_id(provider_id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(provider_id.as_bytes());
    hex::encode(hasher.finalize())
}

fn generate_refresh_token() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

pub async fn find_or_create_user(
    pool: &PgPool,
    provider: &str,
    profile: SocialProfile,
) -> Result<User> {
    let provider_id_hash = hash_provider_id(&profile.provider_id);

    let existing = sqlx::query_as::<_, User>(
        "SELECT u.* FROM users u
         JOIN user_social_accounts sa ON sa.user_id = u.id
         WHERE sa.provider = $1 AND sa.provider_id_hash = $2",
    )
    .bind(provider)
    .bind(&provider_id_hash)
    .fetch_optional(pool)
    .await?;

    if let Some(user) = existing {
        sqlx::query("UPDATE users SET last_login_at = NOW(), updated_at = NOW() WHERE id = $1")
            .bind(user.id)
            .execute(pool)
            .await?;
        return Ok(user);
    }

    let mut tx = pool.begin().await?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, phone, profile_image_url, last_login_at)
         VALUES ($1, $2, $3, $4, NOW())
         RETURNING *",
    )
    .bind(&profile.name)
    .bind(&profile.email)
    .bind(&profile.phone)
    .bind(&profile.profile_image_url)
    .fetch_one(&mut *tx)
    .await?;

    let provider_id_encrypted = encrypt_social_id(&profile.provider_id)?;

    sqlx::query(
        "INSERT INTO user_social_accounts (user_id, provider, provider_id_encrypted, provider_id_hash)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(user.id)
    .bind(provider)
    .bind(&provider_id_encrypted)
    .bind(&provider_id_hash)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(user)
}

pub async fn link_social_account(
    pool: &PgPool,
    user_id: Uuid,
    provider: &str,
    profile: SocialProfile,
) -> Result<()> {
    let provider_id_hash = hash_provider_id(&profile.provider_id);

    let already_linked: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM user_social_accounts WHERE provider = $1 AND provider_id_hash = $2)",
    )
    .bind(provider)
    .bind(&provider_id_hash)
    .fetch_one(pool)
    .await?;

    if already_linked {
        return Err(anyhow!("이미 다른 계정에 연동된 소셜 계정입니다"));
    }

    let provider_id_encrypted = encrypt_social_id(&profile.provider_id)?;

    sqlx::query(
        "INSERT INTO user_social_accounts (user_id, provider, provider_id_encrypted, provider_id_hash)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(user_id)
    .bind(provider)
    .bind(&provider_id_encrypted)
    .bind(&provider_id_hash)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn issue_tokens(pool: &PgPool, user: &User) -> Result<AuthTokens> {
    let access_token = issue_access_token(user.id, &user.role, "")?;

    let raw_refresh = generate_refresh_token();
    let refresh_hash = {
        let mut hasher = Sha256::new();
        hasher.update(raw_refresh.as_bytes());
        hex::encode(hasher.finalize())
    };

    let expiry_days: i64 = std::env::var("JWT_REFRESH_EXPIRY_DAYS")
        .unwrap_or_else(|_| "30".into())
        .parse()
        .unwrap_or(30);

    let expires_at = Utc::now() + Duration::days(expiry_days);

    sqlx::query(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
         VALUES ($1, $2, $3)",
    )
    .bind(user.id)
    .bind(&refresh_hash)
    .bind(expires_at)
    .execute(pool)
    .await?;

    Ok(AuthTokens {
        access_token,
        refresh_token: raw_refresh,
        user: user.clone(),
    })
}

pub async fn refresh_access_token(pool: &PgPool, raw_refresh: &str) -> Result<String> {
    let token_hash = {
        let mut hasher = Sha256::new();
        hasher.update(raw_refresh.as_bytes());
        hex::encode(hasher.finalize())
    };

    let row = sqlx::query_as::<_, crate::domain::refresh_token::RefreshToken>(
        "DELETE FROM refresh_tokens WHERE token_hash = $1 AND expires_at > NOW() RETURNING *",
    )
    .bind(&token_hash)
    .fetch_optional(pool)
    .await?;

    let token_row = row.ok_or_else(|| anyhow!("유효하지 않거나 만료된 refresh token입니다"))?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(token_row.user_id)
        .fetch_one(pool)
        .await?;

    let new_raw_refresh = generate_refresh_token();
    let new_hash = {
        let mut hasher = Sha256::new();
        hasher.update(new_raw_refresh.as_bytes());
        hex::encode(hasher.finalize())
    };
    let expiry_days: i64 = std::env::var("JWT_REFRESH_EXPIRY_DAYS")
        .unwrap_or_else(|_| "30".into())
        .parse()
        .unwrap_or(30);
    let expires_at = Utc::now() + Duration::days(expiry_days);

    sqlx::query(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(user.id)
    .bind(&new_hash)
    .bind(expires_at)
    .execute(pool)
    .await?;

    issue_access_token(user.id, &user.role, "").map_err(|e| anyhow!(e))
}

pub async fn logout(pool: &PgPool, raw_refresh: &str) -> Result<()> {
    let token_hash = {
        let mut hasher = Sha256::new();
        hasher.update(raw_refresh.as_bytes());
        hex::encode(hasher.finalize())
    };
    sqlx::query("DELETE FROM refresh_tokens WHERE token_hash = $1")
        .bind(&token_hash)
        .execute(pool)
        .await?;
    Ok(())
}
