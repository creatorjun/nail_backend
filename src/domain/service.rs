// src/domain/service.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Service {
    pub id: Uuid,
    pub name: String,
    pub duration_minutes: i32,
    pub price_krw: i32,
    pub created_at: Option<DateTime<Utc>>,
}
