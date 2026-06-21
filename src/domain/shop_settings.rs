// src/domain/shop_settings.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::time::Time;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ShopSettings {
    pub id: Uuid,
    pub shop_name: String,
    pub closed_weekdays: Vec<i32>,
    pub open_time: Time,
    pub close_time: Time,
    pub slot_interval_min: i32,
    pub max_booking_days: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
