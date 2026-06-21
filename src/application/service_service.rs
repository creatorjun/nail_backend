// src/application/service_service.rs
use crate::domain::service::Service;
use crate::domain::service_category::ServiceCategory;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_all_categories(pool: &PgPool) -> Result<Vec<ServiceCategory>, sqlx::Error> {
    sqlx::query_as::<_, ServiceCategory>(
        "SELECT * FROM service_categories WHERE is_active = true ORDER BY display_order ASC, created_at ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn get_all_services(pool: &PgPool) -> Result<Vec<Service>, sqlx::Error> {
    sqlx::query_as::<_, Service>(
        "SELECT * FROM services WHERE is_active = true ORDER BY display_order ASC, created_at ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn get_service_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Service>, sqlx::Error> {
    sqlx::query_as::<_, Service>("SELECT * FROM services WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}
