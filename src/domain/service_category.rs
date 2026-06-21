// src/domain/service_category.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServiceCategory {
    pub id: Uuid,
    pub name: String,
    pub display_order: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
