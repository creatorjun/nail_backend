// src/presentation/routes.rs
use axum::{Router, routing::{get, post}};
use sqlx::PgPool;
use super::handlers::{booking_handler, service_handler};

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/services", get(service_handler::list_services))
        .route("/api/services/:id", get(service_handler::get_service))
        .route("/api/bookings", get(booking_handler::list_bookings))
        .route("/api/bookings", post(booking_handler::create_booking))
        .route("/api/bookings/:id", get(booking_handler::get_booking))
        .route("/api/payments/intent", post(booking_handler::create_payment_intent))
        .with_state(pool)
}

async fn health_check() -> &'static str {
    "OK"
}
