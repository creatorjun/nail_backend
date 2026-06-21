// src/domain/shop_settings.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ShopSettings {
    pub id: Uuid,
    pub shop_name: String,
    pub closed_weekdays: Vec<i32>,
    pub open_time: String,
    pub close_time: String,
    pub slot_interval_min: i32,
    pub max_booking_days: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
