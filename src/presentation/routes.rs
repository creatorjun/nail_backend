// src/presentation/routes.rs
use super::handlers::{booking_handler, service_handler};
use axum::{routing::{get, post}, Router};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/categories", get(service_handler::list_categories))
        .route("/api/services", get(service_handler::list_services))
        .route("/api/services/:id", get(service_handler::get_service))
        .route("/api/bookings", get(booking_handler::list_bookings).post(booking_handler::create_booking))
        .route("/api/bookings/:id", get(booking_handler::get_booking))
        .route("/api/bookings/my", get(booking_handler::get_my_bookings))
        .route("/api/bookings/available-slots", get(booking_handler::get_available_slots))
        .with_state(pool)
}

async fn health_check() -> &'static str {
    "OK"
}
