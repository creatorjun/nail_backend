// src/presentation/routes.rs
use super::handlers::{
    admin_handler, auth_handler, booking_handler, payment_handler, service_handler, user_handler,
};
use super::middleware::auth_middleware::{require_admin, require_auth};
use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    let public = Router::new()
        .route("/health", get(health_check))
        // 소셜 로그인
        .route("/auth/naver", get(auth_handler::naver_oauth_url))
        .route("/auth/naver/callback", get(auth_handler::naver_callback))
        .route("/auth/kakao", get(auth_handler::kakao_oauth_url))
        .route("/auth/kakao/callback", get(auth_handler::kakao_callback))
        .route("/auth/refresh", post(auth_handler::refresh_token))
        .route("/auth/logout", post(auth_handler::logout))
        // 공개 API
        .route("/api/categories", get(service_handler::list_categories))
        .route("/api/services", get(service_handler::list_services))
        .route("/api/services/:id", get(service_handler::get_service))
        .route("/api/bookings/available-slots", get(booking_handler::get_available_slots))
        // 결제 콜백 (PG사에서 GET 리다이렉트)
        .route("/api/payments/naver/approve", get(payment_handler::naver_pay_approve))
        .route("/api/payments/kakao/approve", get(payment_handler::kakao_pay_approve))
        .route("/api/payments/kakao/cancel", get(payment_handler::kakao_pay_cancel))
        .route("/api/payments/kakao/fail", get(payment_handler::kakao_pay_fail));

    let user_protected = Router::new()
        // 유저
        .route("/api/users/me", get(user_handler::get_me))
        .route("/api/users/link/naver", get(user_handler::link_naver))
        .route("/api/users/link/kakao", get(user_handler::link_kakao))
        // 예약
        .route("/api/bookings", post(booking_handler::create_booking))
        .route("/api/bookings/my", get(booking_handler::get_my_bookings))
        .route("/api/bookings/:id", get(booking_handler::get_booking))
        .route("/api/bookings/:id/cancel", post(booking_handler::cancel_booking))
        // 결제 요청
        .route("/api/payments/naver/ready", post(payment_handler::naver_pay_ready))
        .route("/api/payments/kakao/ready", post(payment_handler::kakao_pay_ready))
        .layer(middleware::from_fn(require_auth));

    let admin_protected = Router::new()
        .route("/api/admin/categories", get(admin_handler::list_categories).post(admin_handler::create_category))
        .route("/api/admin/categories/:id", put(admin_handler::update_category).delete(admin_handler::delete_category))
        .route("/api/admin/services", get(admin_handler::list_services).post(admin_handler::create_service))
        .route("/api/admin/services/:id", put(admin_handler::update_service).delete(admin_handler::delete_service))
        .route("/api/admin/bookings", get(admin_handler::list_bookings))
        .route("/api/admin/bookings/:id/status", put(admin_handler::update_booking_status))
        .route("/api/admin/bookings/:id/refund", post(admin_handler::process_refund))
        .route("/api/admin/closed-days", get(admin_handler::list_closed_days).post(admin_handler::add_closed_day))
        .route("/api/admin/closed-days/:id", delete(admin_handler::delete_closed_day))
        .route("/api/admin/shop-settings", get(admin_handler::get_shop_settings).put(admin_handler::update_shop_settings))
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
