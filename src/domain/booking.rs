// src/domain/booking.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Booking {
    pub id: Uuid,
    pub service_id: Option<Uuid>,
    pub customer_name: String,
    pub customer_phone: String,
    pub scheduled_at: DateTime<Utc>,
    pub stripe_payment_intent_id: Option<String>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookingRequest {
    pub service_id: Uuid,
    pub customer_name: String,
    pub customer_phone: String,
    pub scheduled_at: DateTime<Utc>,
}
