// src/domain/booking.rs
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Booking {
    pub id: Uuid,
    pub user_id: Uuid,
    pub service_id: Uuid,
    pub scheduled_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub payment_type: String,
    pub status: String,
    pub memo: Option<String>,
    pub admin_memo: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookingRequest {
    pub user_id: Uuid,
    pub service_id: Uuid,
    pub scheduled_at: DateTime<Utc>,
    pub payment_type: String,
    pub memo: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TimeSlot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub available: bool,
}

#[derive(Debug, Serialize)]
pub struct AvailableSlotsResponse {
    pub date: NaiveDate,
    pub slots: Vec<TimeSlot>,
}
