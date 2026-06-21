// src/domain/closed_day.rs
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClosedDay {
    pub id: Uuid,
    pub closed_date: NaiveDate,
    pub r#type: String,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}
