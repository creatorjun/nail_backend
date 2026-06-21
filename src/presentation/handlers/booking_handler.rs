// src/presentation/handlers/booking_handler.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::application::booking_service;
use crate::domain::booking::CreateBookingRequest;
use crate::infrastructure::stripe_client;

#[derive(Deserialize)]
pub struct CreatePaymentIntentRequest {
    pub booking_id: Uuid,
    pub amount_krw: i64,
}

#[derive(Serialize)]
pub struct CreatePaymentIntentResponse {
    pub client_secret: String,
    pub payment_intent_id: String,
}

pub async fn list_bookings(State(pool): State<PgPool>) -> impl IntoResponse {
    match booking_service::get_all_bookings(&pool).await {
        Ok(bookings) => (StatusCode::OK, Json(bookings)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch bookings: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_booking(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match booking_service::get_booking_by_id(&pool, id).await {
        Ok(Some(booking)) => (StatusCode::OK, Json(booking)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch booking: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn create_booking(
    State(pool): State<PgPool>,
    Json(req): Json<CreateBookingRequest>,
) -> impl IntoResponse {
    match booking_service::create_booking(&pool, req).await {
        Ok(booking) => (StatusCode::CREATED, Json(booking)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create booking: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn create_payment_intent(
    State(pool): State<PgPool>,
    Json(req): Json<CreatePaymentIntentRequest>,
) -> impl IntoResponse {
    let client = stripe_client::create_stripe_client();

    match stripe_client::create_payment_intent(&client, req.amount_krw).await {
        Ok(intent) => {
            let client_secret = intent
                .client_secret
                .clone()
                .unwrap_or_default();

            let _ = booking_service::update_booking_status(
                &pool,
                req.booking_id,
                "payment_pending",
                Some(&intent.id),
            )
            .await;

            (StatusCode::OK, Json(CreatePaymentIntentResponse {
                client_secret,
                payment_intent_id: intent.id.to_string(),
            }))
            .into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create payment intent: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
