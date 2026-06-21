// src/domain/user_social_account.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSocialAccount {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub provider_id_encrypted: String,
    pub created_at: DateTime<Utc>,
}
