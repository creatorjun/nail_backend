// src/presentation/handlers/booking_handler.rs
use crate::application::booking_service;
use crate::domain::booking::CreateBookingRequest;
use crate::presentation::middleware::auth_middleware::AuthUser;
use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AvailableSlotsQuery {
    pub date: NaiveDate,
    pub service_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct MyBookingsQuery {
    pub user_id: Option<Uuid>,
}

pub async fn list_bookings(State(pool): State<PgPool>) -> impl IntoResponse {
    match booking_service::get_all_bookings(&pool).await {
        Ok(bookings) => (StatusCode::OK, Json(bookings)).into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_booking(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match booking_service::get_booking_by_id(&pool, id).await {
        Ok(Some(booking)) => (StatusCode::OK, Json(booking)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_my_bookings(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> impl IntoResponse {
    let user_id = match Uuid::parse_str(&auth_user.user_id) {
        Ok(id) => id,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };
    match booking_service::get_my_bookings(&pool, user_id).await {
        Ok(bookings) => (StatusCode::OK, Json(bookings)).into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_available_slots(
    State(pool): State<PgPool>,
    Query(query): Query<AvailableSlotsQuery>,
) -> impl IntoResponse {
    match booking_service::get_available_slots(&pool, query.date, query.service_id).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn create_booking(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Json(mut req): Json<CreateBookingRequest>,
) -> impl IntoResponse {
    let user_id = match Uuid::parse_str(&auth_user.user_id) {
        Ok(id) => id,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };
    req.user_id = user_id;

    match booking_service::create_booking(&pool, req).await {
        Ok(booking) => (StatusCode::CREATED, Json(booking)).into_response(),
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23P01") => {
            (StatusCode::CONFLICT, "이미 예약된 시간입니다").into_response()
        }
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn cancel_booking(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let user_id = match Uuid::parse_str(&auth_user.user_id) {
        Ok(id) => id,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    match booking_service::cancel_booking(&pool, id, user_id).await {
        Ok(Some(booking)) => (StatusCode::OK, Json(booking)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            "취소 가능한 예약을 찾을 수 없습니다",
        ).into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
