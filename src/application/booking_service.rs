// src/application/booking_service.rs
use crate::domain::booking::{Booking, CreateBookingRequest};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_booking(
    pool: &PgPool,
    req: CreateBookingRequest,
) -> Result<Booking, sqlx::Error> {
    sqlx::query_as::<_, Booking>(
        "INSERT INTO bookings (service_id, customer_name, customer_phone, scheduled_at)
         VALUES ($1, $2, $3, $4)
         RETURNING *",
    )
    .bind(req.service_id)
    .bind(req.customer_name)
    .bind(req.customer_phone)
    .bind(req.scheduled_at)
    .fetch_one(pool)
    .await
}

pub async fn get_all_bookings(pool: &PgPool) -> Result<Vec<Booking>, sqlx::Error> {
    sqlx::query_as::<_, Booking>("SELECT * FROM bookings ORDER BY scheduled_at ASC")
        .fetch_all(pool)
        .await
}

pub async fn get_booking_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Booking>, sqlx::Error> {
    sqlx::query_as::<_, Booking>("SELECT * FROM bookings WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn update_booking_status(
    pool: &PgPool,
    id: Uuid,
    status: &str,
    payment_intent_id: Option<&str>,
) -> Result<Option<Booking>, sqlx::Error> {
    sqlx::query_as::<_, Booking>(
        "UPDATE bookings SET status = $1, stripe_payment_intent_id = $2
         WHERE id = $3 RETURNING *",
    )
    .bind(status)
    .bind(payment_intent_id)
    .bind(id)
    .fetch_optional(pool)
    .await
}
