// src/presentation/handlers/admin_handler.rs
use crate::application::admin_service;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub display_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: String,
    pub display_order: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateServiceRequest {
    pub category_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub duration_minutes: i32,
    pub price_krw: i32,
    pub thumbnail_url: Option<String>,
    pub display_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateServiceRequest {
    pub category_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub duration_minutes: i32,
    pub price_krw: i32,
    pub thumbnail_url: Option<String>,
    pub display_order: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBookingStatusRequest {
    pub status: String,
    pub admin_memo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddClosedDayRequest {
    pub closed_date: NaiveDate,
    pub day_type: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateShopSettingsRequest {
    pub shop_name: String,
    pub closed_weekdays: Vec<i32>,
    /// HH:MM 형식 ex) "10:00"
    pub open_time: String,
    /// HH:MM 형식 ex) "20:00"
    pub close_time: String,
    pub slot_interval_min: i32,
    pub max_booking_days: i32,
}

#[derive(Debug, Deserialize)]
pub struct ProcessRefundRequest {
    pub refund_amount_krw: i32,
    pub refund_reason: String,
}

pub async fn list_categories(State(pool): State<PgPool>) -> impl IntoResponse {
    match crate::application::service_service::get_all_categories(&pool).await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn create_category(
    State(pool): State<PgPool>,
    Json(req): Json<CreateCategoryRequest>,
) -> impl IntoResponse {
    match admin_service::create_category(&pool, &req.name, req.display_order.unwrap_or(0)).await {
        Ok(data) => (StatusCode::CREATED, Json(data)).into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_category(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCategoryRequest>,
) -> impl IntoResponse {
    match admin_service::update_category(
        &pool,
        id,
        &req.name,
        req.display_order.unwrap_or(0),
        req.is_active.unwrap_or(true),
    )
    .await
    {
        Ok(Some(data)) => (StatusCode::OK, Json(data)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_category(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match admin_service::delete_category(&pool, id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn list_services(State(pool): State<PgPool>) -> impl IntoResponse {
    match crate::application::service_service::get_all_services(&pool).await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn create_service(
    State(pool): State<PgPool>,
    Json(req): Json<CreateServiceRequest>,
) -> impl IntoResponse {
    match admin_service::create_service(
        &pool,
        req.category_id,
        &req.name,
        req.description.as_deref(),
        req.duration_minutes,
        req.price_krw,
        req.thumbnail_url.as_deref(),
        req.display_order.unwrap_or(0),
    )
    .await
    {
        Ok(data) => (StatusCode::CREATED, Json(data)).into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_service(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateServiceRequest>,
) -> impl IntoResponse {
    match admin_service::update_service(
        &pool,
        id,
        req.category_id,
        &req.name,
        req.description.as_deref(),
        req.duration_minutes,
        req.price_krw,
        req.thumbnail_url.as_deref(),
        req.display_order.unwrap_or(0),
        req.is_active.unwrap_or(true),
    )
    .await
    {
        Ok(Some(data)) => (StatusCode::OK, Json(data)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_service(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match admin_service::delete_service(&pool, id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn list_bookings(State(pool): State<PgPool>) -> impl IntoResponse {
    match admin_service::get_all_bookings_admin(&pool).await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_booking_status(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateBookingStatusRequest>,
) -> impl IntoResponse {
    match admin_service::update_booking_status(&pool, id, &req.status, req.admin_memo.as_deref())
        .await
    {
        Ok(Some(data)) => (StatusCode::OK, Json(data)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn list_closed_days(State(pool): State<PgPool>) -> impl IntoResponse {
    match admin_service::get_all_closed_days(&pool).await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn add_closed_day(
    State(pool): State<PgPool>,
    Json(req): Json<AddClosedDayRequest>,
) -> impl IntoResponse {
    let day_type = req.day_type.as_deref().unwrap_or("TEMPORARY");
    match admin_service::add_closed_day(&pool, req.closed_date, day_type, req.reason.as_deref())
        .await
    {
        Ok(data) => (StatusCode::CREATED, Json(data)).into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_closed_day(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match admin_service::delete_closed_day(&pool, id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_shop_settings(State(pool): State<PgPool>) -> impl IntoResponse {
    match admin_service::get_shop_settings(&pool).await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_shop_settings(
    State(pool): State<PgPool>,
    Json(req): Json<UpdateShopSettingsRequest>,
) -> impl IntoResponse {
    match admin_service::update_shop_settings(
        &pool,
        &req.shop_name,
        req.closed_weekdays,
        &req.open_time,
        &req.close_time,
        req.slot_interval_min,
        req.max_booking_days,
    )
    .await
    {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn process_refund(
    State(pool): State<PgPool>,
    Path(booking_id): Path<Uuid>,
    Json(req): Json<ProcessRefundRequest>,
) -> impl IntoResponse {
    match admin_service::process_refund(
        &pool,
        booking_id,
        req.refund_amount_krw,
        &req.refund_reason,
    )
    .await
    {
        Ok(Some(data)) => (StatusCode::OK, Json(data)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            "결제 완료된 예약을 찾을 수 없습니다",
        )
            .into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
