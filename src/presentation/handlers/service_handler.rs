// src/presentation/handlers/service_handler.rs
use crate::application::service_service;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_categories(State(pool): State<PgPool>) -> impl IntoResponse {
    match service_service::get_all_categories(&pool).await {
        Ok(categories) => (StatusCode::OK, Json(categories)).into_response(),
        Err(error) => {
            tracing::error!("Failed to fetch categories: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn list_services(State(pool): State<PgPool>) -> impl IntoResponse {
    match service_service::get_all_services(&pool).await {
        Ok(services) => (StatusCode::OK, Json(services)).into_response(),
        Err(error) => {
            tracing::error!("Failed to fetch services: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_service(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match service_service::get_service_by_id(&pool, id).await {
        Ok(Some(service)) => (StatusCode::OK, Json(service)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            tracing::error!("Failed to fetch service: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
