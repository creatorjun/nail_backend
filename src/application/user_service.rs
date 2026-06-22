// src/application/user_service.rs
use crate::domain::user::User;
use crate::domain::user_social_account::UserSocialAccount;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 AND is_active = true")
        .bind(user_id)
        .fetch_optional(pool)
        .await
}

pub async fn get_social_accounts(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<UserSocialAccount>, sqlx::Error> {
    sqlx::query_as::<_, UserSocialAccount>(
        "SELECT * FROM user_social_accounts WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}
