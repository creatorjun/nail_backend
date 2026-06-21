// src/presentation/routes.rs
use super::handlers::{auth_handler, booking_handler, service_handler};
use super::middleware::auth_middleware::{require_admin, require_auth};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    let public = Router::new()
        .route("/health", get(health_check))
        .route("/auth/naver", get(auth_handler::naver_oauth_url))
        .route("/auth/naver/callback", get(auth_handler::naver_callback))
        .route("/auth/kakao", get(auth_handler::kakao_oauth_url))
        .route("/auth/kakao/callback", get(auth_handler::kakao_callback))
        .route("/auth/refresh", post(auth_handler::refresh_token))
        .route("/auth/logout", post(auth_handler::logout))
        .route("/api/categories", get(service_handler::list_categories))
        .route("/api/services", get(service_handler::list_services))
        .route("/api/services/:id", get(service_handler::get_service))
        .route("/api/bookings/available-slots", get(booking_handler::get_available_slots));

    let user_protected = Router::new()
        .route("/api/bookings", post(booking_handler::create_booking))
        .route("/api/bookings/my", get(booking_handler::get_my_bookings))
        .route("/api/bookings/:id", get(booking_handler::get_booking))
        .layer(middleware::from_fn(require_auth));

    let admin_protected = Router::new()
        .route("/api/admin/bookings", get(booking_handler::list_bookings))
        .layer(middleware::from_fn(require_admin));

    Router::new()
        .merge(public)
        .merge(user_protected)
        .merge(admin_protected)
        .with_state(pool)
}

async fn health_check() -> &'static str {
    "OK"
}
