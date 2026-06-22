// src/application/admin_service.rs
use crate::domain::booking::Booking;
use crate::domain::closed_day::ClosedDay;
use crate::domain::payment::Payment;
use crate::domain::service::Service;
use crate::domain::service_category::ServiceCategory;
use crate::domain::shop_settings::ShopSettings;
use crate::application::payment_service;
use anyhow::Result;
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_category(
    pool: &PgPool,
    name: &str,
    display_order: i32,
) -> Result<ServiceCategory, sqlx::Error> {
    sqlx::query_as::<_, ServiceCategory>(
        "INSERT INTO service_categories (name, display_order) VALUES ($1, $2) RETURNING *",
    )
    .bind(name)
    .bind(display_order)
    .fetch_one(pool)
    .await
}

pub async fn update_category(
    pool: &PgPool,
    id: Uuid,
    name: &str,
    display_order: i32,
    is_active: bool,
) -> Result<Option<ServiceCategory>, sqlx::Error> {
    sqlx::query_as::<_, ServiceCategory>(
        "UPDATE service_categories SET name=$1, display_order=$2, is_active=$3 WHERE id=$4 RETURNING *",
    )
    .bind(name)
    .bind(display_order)
    .bind(is_active)
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_category(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM service_categories WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn create_service(
    pool: &PgPool,
    category_id: Option<Uuid>,
    name: &str,
    description: Option<&str>,
    duration_minutes: i32,
    price_krw: i32,
    thumbnail_url: Option<&str>,
    display_order: i32,
) -> Result<Service, sqlx::Error> {
    sqlx::query_as::<_, Service>(
        "INSERT INTO services (category_id, name, description, duration_minutes, price_krw, thumbnail_url, display_order)
         VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
    )
    .bind(category_id)
    .bind(name)
    .bind(description)
    .bind(duration_minutes)
    .bind(price_krw)
    .bind(thumbnail_url)
    .bind(display_order)
    .fetch_one(pool)
    .await
}

pub async fn update_service(
    pool: &PgPool,
    id: Uuid,
    category_id: Option<Uuid>,
    name: &str,
    description: Option<&str>,
    duration_minutes: i32,
    price_krw: i32,
    thumbnail_url: Option<&str>,
    display_order: i32,
    is_active: bool,
) -> Result<Option<Service>, sqlx::Error> {
    sqlx::query_as::<_, Service>(
        "UPDATE services SET category_id=$1, name=$2, description=$3, duration_minutes=$4,
         price_krw=$5, thumbnail_url=$6, display_order=$7, is_active=$8, updated_at=NOW()
         WHERE id=$9 RETURNING *",
    )
    .bind(category_id)
    .bind(name)
    .bind(description)
    .bind(duration_minutes)
    .bind(price_krw)
    .bind(thumbnail_url)
    .bind(display_order)
    .bind(is_active)
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_service(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result =
        sqlx::query("UPDATE services SET is_active = false, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn get_all_bookings_admin(pool: &PgPool) -> Result<Vec<Booking>, sqlx::Error> {
    sqlx::query_as::<_, Booking>("SELECT * FROM bookings ORDER BY scheduled_at ASC")
        .fetch_all(pool)
        .await
}

pub async fn update_booking_status(
    pool: &PgPool,
    id: Uuid,
    status: &str,
    admin_memo: Option<&str>,
) -> Result<Option<Booking>, sqlx::Error> {
    sqlx::query_as::<_, Booking>(
        "UPDATE bookings SET status=$1, admin_memo=COALESCE($2, admin_memo), updated_at=NOW()
         WHERE id=$3 RETURNING *",
    )
    .bind(status)
    .bind(admin_memo)
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn add_closed_day(
    pool: &PgPool,
    closed_date: NaiveDate,
    day_type: &str,
    reason: Option<&str>,
) -> Result<ClosedDay, sqlx::Error> {
    sqlx::query_as::<_, ClosedDay>(
        "INSERT INTO closed_days (closed_date, type, reason) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(closed_date)
    .bind(day_type)
    .bind(reason)
    .fetch_one(pool)
    .await
}

pub async fn delete_closed_day(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM closed_days WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn get_all_closed_days(pool: &PgPool) -> Result<Vec<ClosedDay>, sqlx::Error> {
    sqlx::query_as::<_, ClosedDay>("SELECT * FROM closed_days ORDER BY closed_date ASC")
        .fetch_all(pool)
        .await
}

pub async fn get_shop_settings(pool: &PgPool) -> Result<ShopSettings, sqlx::Error> {
    sqlx::query_as::<_, ShopSettings>(
        "SELECT id, shop_name, closed_weekdays,
                open_time::TEXT, close_time::TEXT,
                slot_interval_min, max_booking_days,
                created_at, updated_at
         FROM shop_settings LIMIT 1",
    )
    .fetch_one(pool)
    .await
}

pub async fn update_shop_settings(
    pool: &PgPool,
    shop_name: &str,
    closed_weekdays: Vec<i32>,
    open_time: &str,
    close_time: &str,
    slot_interval_min: i32,
    max_booking_days: i32,
) -> Result<ShopSettings, sqlx::Error> {
    sqlx::query_as::<_, ShopSettings>(
        "UPDATE shop_settings
         SET shop_name=$1, closed_weekdays=$2,
             open_time=$3::TIME, close_time=$4::TIME,
             slot_interval_min=$5, max_booking_days=$6,
             updated_at=NOW()
         RETURNING id, shop_name, closed_weekdays,
                   open_time::TEXT, close_time::TEXT,
                   slot_interval_min, max_booking_days,
                   created_at, updated_at",
    )
    .bind(shop_name)
    .bind(closed_weekdays)
    .bind(open_time)
    .bind(close_time)
    .bind(slot_interval_min)
    .bind(max_booking_days)
    .fetch_one(pool)
    .await
}

/// PG사 환불 API 실제 호출 후 DB 상태 변경
pub async fn process_refund(
    pool: &PgPool,
    booking_id: Uuid,
    refund_amount_krw: i32,
    refund_reason: &str,
) -> Result<Payment> {
    payment_service::refund_payment(pool, booking_id, refund_amount_krw, refund_reason).await
}
