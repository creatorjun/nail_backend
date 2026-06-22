// src/application/payment_service.rs
use crate::domain::payment::Payment;
use crate::infrastructure::payment::{kakao_pay, naver_pay};
use anyhow::{anyhow, Result};
use sqlx::PgPool;
use uuid::Uuid;

// ── 공통 준비 요청/응답 ────────────────────────────────────────

pub struct PaymentReadyResult {
    pub pg_order_id: String,
    /// 네이버페이: payExpressUrl / 카카오페이: next_redirect_app_url
    pub payment_url: String,
    /// 카카오페이 tid (승인 시 필요)
    pub tid: Option<String>,
}

// ── 네이버페이 준비 ────────────────────────────────────────────

pub async fn naver_pay_ready(
    pool: &PgPool,
    booking_id: Uuid,
    user_id: Uuid,
    product_name: &str,
    amount: i32,
    return_url: &str,
) -> Result<PaymentReadyResult> {
    let pg_order_id = format!("NAVER-{}", Uuid::new_v4());

    let res = naver_pay::ready(naver_pay::NaverPayReadyRequest {
        merchant_user_key: user_id.to_string(),
        merchant_pay_key: pg_order_id.clone(),
        product_name: product_name.to_string(),
        total_pay_amount: amount,
        tax_scope_amount: amount,
        tax_ex_scope_amount: 0,
        return_url: return_url.to_string(),
    })
    .await?;

    let payment_url = res
        .pay_express_url
        .or(res.pay_highlight_url)
        .ok_or_else(|| anyhow!("NaverPay: no payment URL in response"))?;

    sqlx::query(
        "INSERT INTO payments (booking_id, user_id, provider, payment_type, amount_krw, status, pg_order_id)
         VALUES ($1, $2, 'NAVER_PAY', 'INSTANT', $3, 'READY', $4)",
    )
    .bind(booking_id)
    .bind(user_id)
    .bind(amount)
    .bind(&pg_order_id)
    .execute(pool)
    .await?;

    Ok(PaymentReadyResult {
        pg_order_id,
        payment_url,
        tid: None,
    })
}

// ── 네이버페이 승인 ────────────────────────────────────────────

pub async fn naver_pay_approve(
    pool: &PgPool,
    pg_order_id: &str,
    merchant_user_key: &str,
    payment_id: &str,
) -> Result<Payment> {
    naver_pay::approve(naver_pay::NaverPayApproveRequest {
        merchant_pay_key: pg_order_id.to_string(),
        merchant_user_key: merchant_user_key.to_string(),
        payment_id: payment_id.to_string(),
    })
    .await?;

    let payment = sqlx::query_as::<_, Payment>(
        "UPDATE payments
         SET status = 'PAID', pg_transaction_id = $1, paid_at = NOW(), updated_at = NOW()
         WHERE pg_order_id = $2
         RETURNING *",
    )
    .bind(payment_id)
    .bind(pg_order_id)
    .fetch_one(pool)
    .await?;

    sqlx::query(
        "UPDATE bookings SET status = 'CONFIRMED', updated_at = NOW() WHERE id = $1",
    )
    .bind(payment.booking_id)
    .execute(pool)
    .await?;

    Ok(payment)
}

// ── 카카오페이 준비 ────────────────────────────────────────────

pub async fn kakao_pay_ready(
    pool: &PgPool,
    booking_id: Uuid,
    user_id: Uuid,
    product_name: &str,
    amount: i32,
) -> Result<PaymentReadyResult> {
    let pg_order_id = format!("KAKAO-{}", Uuid::new_v4());
    let cid = std::env::var("KAKAO_PAY_CID").unwrap_or_else(|_| "TC0ONETIME".into());
    let approval_url = std::env::var("KAKAO_PAY_APPROVAL_URL")?;
    let cancel_url = std::env::var("KAKAO_PAY_CANCEL_URL")?;
    let fail_url = std::env::var("KAKAO_PAY_FAIL_URL")?;

    let res = kakao_pay::ready(kakao_pay::KakaoPayReadyRequest {
        cid: cid.clone(),
        partner_order_id: pg_order_id.clone(),
        partner_user_id: user_id.to_string(),
        item_name: product_name.to_string(),
        quantity: 1,
        total_amount: amount,
        tax_free_amount: 0,
        approval_url,
        cancel_url,
        fail_url,
    })
    .await?;

    let payment_url = res
        .next_redirect_app_url
        .or(res.next_redirect_mobile_url)
        .or(res.next_redirect_pc_url)
        .ok_or_else(|| anyhow!("KakaoPay: no redirect URL in response"))?;

    sqlx::query(
        "INSERT INTO payments (booking_id, user_id, provider, payment_type, amount_krw, status, pg_order_id, pg_transaction_id)
         VALUES ($1, $2, 'KAKAO_PAY', 'INSTANT', $3, 'READY', $4, $5)",
    )
    .bind(booking_id)
    .bind(user_id)
    .bind(amount)
    .bind(&pg_order_id)
    .bind(&res.tid)
    .execute(pool)
    .await?;

    Ok(PaymentReadyResult {
        pg_order_id,
        payment_url,
        tid: Some(res.tid),
    })
}

// ── 카카오페이 승인 ────────────────────────────────────────────

pub async fn kakao_pay_approve(
    pool: &PgPool,
    pg_order_id: &str,
    pg_token: &str,
) -> Result<Payment> {
    let cid = std::env::var("KAKAO_PAY_CID").unwrap_or_else(|_| "TC0ONETIME".into());

    let row = sqlx::query_as::<_, Payment>(
        "SELECT * FROM payments WHERE pg_order_id = $1",
    )
    .bind(pg_order_id)
    .fetch_one(pool)
    .await?;

    let tid = row
        .pg_transaction_id
        .as_deref()
        .ok_or_else(|| anyhow!("KakaoPay: tid not found for order {}", pg_order_id))?;

    kakao_pay::approve(kakao_pay::KakaoPayApproveRequest {
        cid,
        tid: tid.to_string(),
        partner_order_id: pg_order_id.to_string(),
        partner_user_id: row.user_id.to_string(),
        pg_token: pg_token.to_string(),
    })
    .await?;

    let payment = sqlx::query_as::<_, Payment>(
        "UPDATE payments
         SET status = 'PAID', paid_at = NOW(), updated_at = NOW()
         WHERE pg_order_id = $1
         RETURNING *",
    )
    .bind(pg_order_id)
    .fetch_one(pool)
    .await?;

    sqlx::query(
        "UPDATE bookings SET status = 'CONFIRMED', updated_at = NOW() WHERE id = $1",
    )
    .bind(payment.booking_id)
    .execute(pool)
    .await?;

    Ok(payment)
}

// ── 공통 환불 (admin_service에서 호출) ────────────────────────

pub async fn refund_payment(
    pool: &PgPool,
    booking_id: Uuid,
    refund_amount: i32,
    refund_reason: &str,
) -> Result<Payment> {
    let payment = sqlx::query_as::<_, Payment>(
        "SELECT * FROM payments WHERE booking_id = $1 AND status = 'PAID'",
    )
    .bind(booking_id)
    .fetch_one(pool)
    .await?;

    match payment.provider.as_str() {
        "NAVER_PAY" => {
            let payment_id = payment
                .pg_transaction_id
                .as_deref()
                .ok_or_else(|| anyhow!("NaverPay: no transaction id"))?;
            naver_pay::cancel(naver_pay::NaverPayCancelRequest {
                payment_id: payment_id.to_string(),
                cancel_amount: refund_amount,
                cancel_reason: refund_reason.to_string(),
                cancel_require_point: None,
            })
            .await?;
        }
        "KAKAO_PAY" => {
            let cid = std::env::var("KAKAO_PAY_CID").unwrap_or_else(|_| "TC0ONETIME".into());
            let tid = payment
                .pg_transaction_id
                .as_deref()
                .ok_or_else(|| anyhow!("KakaoPay: no tid"))?;
            kakao_pay::cancel(kakao_pay::KakaoPayCancelRequest {
                cid,
                tid: tid.to_string(),
                cancel_amount: refund_amount,
                cancel_tax_free_amount: 0,
            })
            .await?;
        }
        other => return Err(anyhow!("Unknown payment provider: {}", other)),
    }

    let new_status = if refund_amount == payment.amount_krw { "CANCELLED" } else { "PARTIAL_CANCELLED" };

    let updated = sqlx::query_as::<_, Payment>(
        "UPDATE payments
         SET status = $1, refund_amount_krw = $2, refund_reason = $3,
             refunded_at = NOW(), updated_at = NOW()
         WHERE id = $4
         RETURNING *",
    )
    .bind(new_status)
    .bind(refund_amount)
    .bind(refund_reason)
    .bind(payment.id)
    .fetch_one(pool)
    .await?;

    sqlx::query(
        "UPDATE bookings SET status = 'CANCELLED_BY_ADMIN', updated_at = NOW() WHERE id = $1",
    )
    .bind(booking_id)
    .execute(pool)
    .await?;

    Ok(updated)
}
