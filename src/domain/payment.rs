// src/domain/payment.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Payment {
    pub id: Uuid,
    pub booking_id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub payment_type: String,
    pub amount_krw: i32,
    pub status: String,
    pub pg_order_id: String,
    pub pg_transaction_id: Option<String>,
    pub paid_at: Option<DateTime<Utc>>,
    pub refund_amount_krw: Option<i32>,
    pub refund_reason: Option<String>,
    pub refunded_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
