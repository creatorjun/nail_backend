// src/presentation/handlers/payment_handler.rs
use crate::application::payment_service;
use crate::presentation::middleware::auth_middleware::AuthUser;
use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

// ── 요청/응답 타입 ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct NaverPayReadyRequest {
    pub booking_id: Uuid,
    pub product_name: String,
    pub amount: i32,
    pub return_url: String,
}

#[derive(Debug, Deserialize)]
pub struct KakaoPayReadyRequest {
    pub booking_id: Uuid,
    pub product_name: String,
    pub amount: i32,
}

#[derive(Debug, Serialize)]
pub struct PaymentReadyResponse {
    pub pg_order_id: String,
    pub payment_url: String,
}

#[derive(Debug, Deserialize)]
pub struct NaverPayApproveQuery {
    pub pg_order_id: String,
    pub merchant_user_key: String,
    pub payment_id: String,
}

#[derive(Debug, Deserialize)]
pub struct KakaoPayApproveQuery {
    pub pg_token: String,
    pub partner_order_id: String,
}

// ── 네이버페이 준비 ────────────────────────────────────────────

pub async fn naver_pay_ready(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<NaverPayReadyRequest>,
) -> impl IntoResponse {
    let user_id = match Uuid::parse_str(&auth_user.user_id) {
        Ok(id) => id,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    match payment_service::naver_pay_ready(
        &pool,
        req.booking_id,
        user_id,
        &req.product_name,
        req.amount,
        &req.return_url,
    )
    .await
    {
        Ok(result) => (StatusCode::OK, Json(PaymentReadyResponse {
            pg_order_id: result.pg_order_id,
            payment_url: result.payment_url,
        })).into_response(),
        Err(e) => {
            tracing::error!("NaverPay ready error: {:?}", e);
            (StatusCode::BAD_GATEWAY, e.to_string()).into_response()
        }
    }
}

// ── 네이버페이 승인 콜백 ───────────────────────────────────────
// 네이버페이는 return_url로 GET 리다이렉트됨

pub async fn naver_pay_approve(
    State(pool): State<PgPool>,
    Query(q): Query<NaverPayApproveQuery>,
) -> impl IntoResponse {
    match payment_service::naver_pay_approve(
        &pool,
        &q.pg_order_id,
        &q.merchant_user_key,
        &q.payment_id,
    )
    .await
    {
        Ok(payment) => (StatusCode::OK, Json(payment)).into_response(),
        Err(e) => {
            tracing::error!("NaverPay approve error: {:?}", e);
            (StatusCode::BAD_GATEWAY, e.to_string()).into_response()
        }
    }
}

// ── 카카오페이 준비 ────────────────────────────────────────────

pub async fn kakao_pay_ready(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<KakaoPayReadyRequest>,
) -> impl IntoResponse {
    let user_id = match Uuid::parse_str(&auth_user.user_id) {
        Ok(id) => id,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    match payment_service::kakao_pay_ready(
        &pool,
        req.booking_id,
        user_id,
        &req.product_name,
        req.amount,
    )
    .await
    {
        Ok(result) => (StatusCode::OK, Json(PaymentReadyResponse {
            pg_order_id: result.pg_order_id,
            payment_url: result.payment_url,
        })).into_response(),
        Err(e) => {
            tracing::error!("KakaoPay ready error: {:?}", e);
            (StatusCode::BAD_GATEWAY, e.to_string()).into_response()
        }
    }
}

// ── 카카오페이 승인 콜백 ───────────────────────────────────────
// approval_url로 GET 리다이렉트됨 (pg_token 포함)

pub async fn kakao_pay_approve(
    State(pool): State<PgPool>,
    Query(q): Query<KakaoPayApproveQuery>,
) -> impl IntoResponse {
    match payment_service::kakao_pay_approve(&pool, &q.partner_order_id, &q.pg_token).await {
        Ok(payment) => (StatusCode::OK, Json(payment)).into_response(),
        Err(e) => {
            tracing::error!("KakaoPay approve error: {:?}", e);
            (StatusCode::BAD_GATEWAY, e.to_string()).into_response()
        }
    }
}

// ── 카카오페이 취소/실패 콜백 ─────────────────────────────────

pub async fn kakao_pay_cancel() -> impl IntoResponse {
    (StatusCode::OK, "payment cancelled by user")
}

pub async fn kakao_pay_fail() -> impl IntoResponse {
    (StatusCode::OK, "payment failed")
}
