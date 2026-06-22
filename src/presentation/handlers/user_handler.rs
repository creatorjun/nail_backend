// src/presentation/handlers/user_handler.rs
use crate::application::{auth_service, user_service};
use crate::infrastructure::social::{kakao, naver};
use crate::presentation::middleware::auth_middleware::AuthUser;
use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub profile_image_url: Option<String>,
    pub role: String,
    pub linked_providers: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct LinkCallbackQuery {
    pub code: String,
    pub state: Option<String>,
}

pub async fn get_me(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> impl IntoResponse {
    let user_id = match Uuid::parse_str(&auth_user.user_id) {
        Ok(id) => id,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let user = match user_service::get_user_by_id(&pool, user_id).await {
        Ok(Some(u)) => u,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("{:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let accounts = match user_service::get_social_accounts(&pool, user_id).await {
        Ok(a) => a,
        Err(e) => {
            tracing::error!("{:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let linked_providers = accounts.into_iter().map(|a| a.provider).collect();

    (StatusCode::OK, Json(MeResponse {
        id: user.id.to_string(),
        name: user.name,
        email: user.email,
        phone: user.phone,
        profile_image_url: user.profile_image_url,
        role: user.role,
        linked_providers,
    })).into_response()
}

pub async fn link_naver(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Query(q): Query<LinkCallbackQuery>,
) -> impl IntoResponse {
    let user_id = match Uuid::parse_str(&auth_user.user_id) {
        Ok(id) => id,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let state = q.state.as_deref().unwrap_or("");
    match naver::fetch_profile(&q.code, state).await {
        Ok(profile) => match auth_service::link_social_account(&pool, user_id, "NAVER", profile).await {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(e) => (StatusCode::CONFLICT, e.to_string()).into_response(),
        },
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::BAD_GATEWAY.into_response()
        }
    }
}

pub async fn link_kakao(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Query(q): Query<LinkCallbackQuery>,
) -> impl IntoResponse {
    let user_id = match Uuid::parse_str(&auth_user.user_id) {
        Ok(id) => id,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    match kakao::fetch_profile(&q.code).await {
        Ok(profile) => match auth_service::link_social_account(&pool, user_id, "KAKAO", profile).await {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(e) => (StatusCode::CONFLICT, e.to_string()).into_response(),
        },
        Err(e) => {
            tracing::error!("{:?}", e);
            StatusCode::BAD_GATEWAY.into_response()
        }
    }
}
