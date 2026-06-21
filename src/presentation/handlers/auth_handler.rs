// src/presentation/handlers/auth_handler.rs
use crate::application::auth_service;
use crate::infrastructure::social::{kakao, naver};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OAuthStateQuery {
    pub state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
}

pub async fn naver_oauth_url(Query(q): Query<OAuthStateQuery>) -> impl IntoResponse {
    let state = q.state.unwrap_or_else(|| Uuid::new_v4().to_string());
    let url = naver::get_oauth_url(&state);
    Json(serde_json::json!({ "url": url }))
}

pub async fn naver_callback(
    State(pool): State<PgPool>,
    Query(q): Query<OAuthCallbackQuery>,
) -> impl IntoResponse {
    let state = q.state.as_deref().unwrap_or("");
    match naver::fetch_profile(&q.code, state).await {
        Ok(profile) => match auth_service::find_or_create_user(&pool, "NAVER", profile).await {
            Ok(user) => match auth_service::issue_tokens(&pool, &user).await {
                Ok(tokens) => (StatusCode::OK, Json(AuthResponse {
                    access_token: tokens.access_token,
                    refresh_token: tokens.refresh_token,
                    user_id: user.id.to_string(),
                    name: user.name,
                    role: user.role,
                })).into_response(),
                Err(e) => {
                    tracing::error!("Failed to issue tokens: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            },
            Err(e) => {
                tracing::error!("Failed to find or create user: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        },
        Err(e) => {
            tracing::error!("Naver fetch profile failed: {:?}", e);
            StatusCode::BAD_GATEWAY.into_response()
        }
    }
}

pub async fn kakao_oauth_url(Query(q): Query<OAuthStateQuery>) -> impl IntoResponse {
    let state = q.state.unwrap_or_else(|| Uuid::new_v4().to_string());
    let url = kakao::get_oauth_url(&state);
    Json(serde_json::json!({ "url": url }))
}

pub async fn kakao_callback(
    State(pool): State<PgPool>,
    Query(q): Query<OAuthCallbackQuery>,
) -> impl IntoResponse {
    match kakao::fetch_profile(&q.code).await {
        Ok(profile) => match auth_service::find_or_create_user(&pool, "KAKAO", profile).await {
            Ok(user) => match auth_service::issue_tokens(&pool, &user).await {
                Ok(tokens) => (StatusCode::OK, Json(AuthResponse {
                    access_token: tokens.access_token,
                    refresh_token: tokens.refresh_token,
                    user_id: user.id.to_string(),
                    name: user.name,
                    role: user.role,
                })).into_response(),
                Err(e) => {
                    tracing::error!("Failed to issue tokens: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            },
            Err(e) => {
                tracing::error!("Failed to find or create user: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        },
        Err(e) => {
            tracing::error!("Kakao fetch profile failed: {:?}", e);
            StatusCode::BAD_GATEWAY.into_response()
        }
    }
}

pub async fn refresh_token(
    State(pool): State<PgPool>,
    Json(req): Json<RefreshRequest>,
) -> impl IntoResponse {
    match auth_service::refresh_access_token(&pool, &req.refresh_token).await {
        Ok(access_token) => (StatusCode::OK, Json(AccessTokenResponse { access_token })).into_response(),
        Err(e) => {
            tracing::warn!("Refresh token failed: {:?}", e);
            (StatusCode::UNAUTHORIZED, e.to_string()).into_response()
        }
    }
}

pub async fn logout(
    State(pool): State<PgPool>,
    Json(req): Json<LogoutRequest>,
) -> impl IntoResponse {
    match auth_service::logout(&pool, &req.refresh_token).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            tracing::error!("Logout failed: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
