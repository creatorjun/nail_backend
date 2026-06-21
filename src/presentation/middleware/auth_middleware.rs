// src/presentation/middleware/auth_middleware.rs
use crate::infrastructure::jwt::{verify_access_token, Claims};
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: String,
    pub role: String,
}

pub async fn require_auth(mut req: Request, next: Next) -> Response {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let token = match token {
        Some(t) => t.to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Authorization header missing").into_response(),
    };

    match verify_access_token(&token) {
        Ok(claims) => {
            req.extensions_mut().insert(AuthUser {
                user_id: claims.sub,
                role: claims.role,
            });
            next.run(req).await
        }
        Err(_) => (StatusCode::UNAUTHORIZED, "Invalid or expired token").into_response(),
    }
}

pub async fn require_admin(mut req: Request, next: Next) -> Response {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let token = match token {
        Some(t) => t.to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Authorization header missing").into_response(),
    };

    match verify_access_token(&token) {
        Ok(claims) if claims.role == "ADMIN" => {
            req.extensions_mut().insert(AuthUser {
                user_id: claims.sub,
                role: claims.role,
            });
            next.run(req).await
        }
        Ok(_) => (StatusCode::FORBIDDEN, "Admin only").into_response(),
        Err(_) => (StatusCode::UNAUTHORIZED, "Invalid or expired token").into_response(),
    }
}
