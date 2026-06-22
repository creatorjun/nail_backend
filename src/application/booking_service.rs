// src/application/booking_service.rs
use crate::domain::booking::{AvailableSlotsResponse, Booking, CreateBookingRequest, TimeSlot};
use crate::domain::service::Service;
use crate::domain::shop_settings::ShopSettings;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_booking(pool: &PgPool, req: CreateBookingRequest) -> Result<Booking, sqlx::Error> {
    let service = sqlx::query_as::<_, Service>("SELECT * FROM services WHERE id = $1 AND is_active = true")
        .bind(req.service_id)
        .fetch_one(pool)
        .await?;

    let end_at = req.scheduled_at + Duration::minutes(service.duration_minutes as i64);
    let status = if req.payment_type == "ONSITE" { "CONFIRMED" } else { "PENDING" };

    sqlx::query_as::<_, Booking>(
        "INSERT INTO bookings (user_id, service_id, scheduled_at, end_at, payment_type, status, memo)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *",
    )
    .bind(req.user_id)
    .bind(req.service_id)
    .bind(req.scheduled_at)
    .bind(end_at)
    .bind(req.payment_type)
    .bind(status)
    .bind(req.memo)
    .fetch_one(pool)
    .await
}

pub async fn get_my_bookings(pool: &PgPool, user_id: Uuid) -> Result<Vec<Booking>, sqlx::Error> {
    sqlx::query_as::<_, Booking>(
        "SELECT * FROM bookings WHERE user_id = $1 ORDER BY scheduled_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_all_bookings(pool: &PgPool) -> Result<Vec<Booking>, sqlx::Error> {
    sqlx::query_as::<_, Booking>("SELECT * FROM bookings ORDER BY scheduled_at ASC")
        .fetch_all(pool)
        .await
}

pub async fn get_booking_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Booking>, sqlx::Error> {
    sqlx::query_as::<_, Booking>("SELECT * FROM bookings WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn cancel_booking(
    pool: &PgPool,
    booking_id: Uuid,
    user_id: Uuid,
) -> Result<Option<Booking>, sqlx::Error> {
    sqlx::query_as::<_, Booking>(
        "UPDATE bookings
         SET status = 'CANCELLED_BY_USER', updated_at = NOW()
         WHERE id = $1
           AND user_id = $2
           AND status NOT IN ('CANCELLED_BY_USER', 'CANCELLED_BY_ADMIN', 'COMPLETED')
         RETURNING *",
    )
    .bind(booking_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

fn parse_time(s: &str) -> Option<NaiveTime> {
    NaiveTime::parse_from_str(s, "%H:%M")
        .or_else(|_| NaiveTime::parse_from_str(s, "%H:%M:%S"))
        .ok()
}

pub async fn get_available_slots(
    pool: &PgPool,
    date: NaiveDate,
    service_id: Uuid,
) -> Result<AvailableSlotsResponse, sqlx::Error> {
    let settings = sqlx::query_as::<_, ShopSettings>(
        "SELECT id, shop_name, closed_weekdays,
                open_time::TEXT, close_time::TEXT,
                slot_interval_min, max_booking_days,
                created_at, updated_at
         FROM shop_settings LIMIT 1",
    )
    .fetch_one(pool)
    .await?;

    let service = sqlx::query_as::<_, Service>("SELECT * FROM services WHERE id = $1 AND is_active = true")
        .bind(service_id)
        .fetch_one(pool)
        .await?;

    let weekday = date.weekday().num_days_from_sunday() as i32;
    let is_closed_weekday = settings.closed_weekdays.iter().any(|day| *day == weekday);

    let is_closed_day: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM closed_days WHERE closed_date = $1)")
            .bind(date)
            .fetch_one(pool)
            .await?;

    if is_closed_weekday || is_closed_day {
        return Ok(AvailableSlotsResponse { date, slots: vec![] });
    }

    let open_naive = parse_time(&settings.open_time)
        .unwrap_or_else(|| NaiveTime::from_hms_opt(10, 0, 0).unwrap());
    let close_naive = parse_time(&settings.close_time)
        .unwrap_or_else(|| NaiveTime::from_hms_opt(20, 0, 0).unwrap());

    let day_start = Utc.from_utc_datetime(&NaiveDateTime::new(date, open_naive));
    let day_end = Utc.from_utc_datetime(&NaiveDateTime::new(date, close_naive));

    let bookings = sqlx::query_as::<_, Booking>(
        "SELECT * FROM bookings
         WHERE scheduled_at < $2
           AND end_at > $1
           AND status NOT IN ('CANCELLED_BY_USER', 'CANCELLED_BY_ADMIN')
         ORDER BY scheduled_at ASC",
    )
    .bind(day_start)
    .bind(day_end)
    .fetch_all(pool)
    .await?;

    let mut slots = Vec::new();
    let mut current = day_start;
    let service_duration = Duration::minutes(service.duration_minutes as i64);
    let interval = Duration::minutes(settings.slot_interval_min as i64);

    while current + service_duration <= day_end {
        let candidate_end = current + service_duration;
        let overlapped = bookings
            .iter()
            .any(|b| current < b.end_at && candidate_end > b.scheduled_at);

        slots.push(TimeSlot {
            start: current,
            end: candidate_end,
            available: !overlapped,
        });

        current += interval;
    }

    Ok(AvailableSlotsResponse { date, slots })
}
