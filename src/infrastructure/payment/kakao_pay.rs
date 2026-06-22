// src/infrastructure/payment/kakao_pay.rs
//
// 카카오페이 단건결제 API
// 문서: https://developers.kakaopay.com/docs/payment/online/single-payment

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::env;

fn client() -> reqwest::Client {
    reqwest::Client::new()
}

// ── 결제 준비 ──────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct KakaoPayReadyRequest {
    pub cid: String,
    pub partner_order_id: String,
    pub partner_user_id: String,
    pub item_name: String,
    pub quantity: i32,
    pub total_amount: i32,
    pub tax_free_amount: i32,
    pub approval_url: String,
    pub cancel_url: String,
    pub fail_url: String,
}

#[derive(Debug, Deserialize)]
pub struct KakaoPayReadyResponse {
    pub tid: String,
    pub next_redirect_app_url: Option<String>,
    pub next_redirect_mobile_url: Option<String>,
    pub next_redirect_pc_url: Option<String>,
    pub created_at: String,
}

pub async fn ready(req: KakaoPayReadyRequest) -> Result<KakaoPayReadyResponse> {
    let secret_key = env::var("KAKAO_PAY_SECRET_KEY")?;

    let res = client()
        .post("https://open-api.kakaopay.com/online/v1/payment/ready")
        .header("Authorization", format!("SECRET_KEY {}", secret_key))
        .json(&req)
        .send()
        .await?;

    if !res.status().is_success() {
        let text = res.text().await.unwrap_or_default();
        return Err(anyhow!("KakaoPay ready failed: {}", text));
    }

    Ok(res.json().await?)
}

// ── 결제 승인 ──────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct KakaoPayApproveRequest {
    pub cid: String,
    pub tid: String,
    pub partner_order_id: String,
    pub partner_user_id: String,
    pub pg_token: String,
}

#[derive(Debug, Deserialize)]
pub struct KakaoPayAmount {
    pub total: i32,
    pub tax_free: i32,
    pub vat: i32,
}

#[derive(Debug, Deserialize)]
pub struct KakaoPayApproveResponse {
    pub tid: String,
    pub partner_order_id: String,
    pub partner_user_id: String,
    pub payment_method_type: String,
    pub amount: KakaoPayAmount,
    pub approved_at: String,
}

pub async fn approve(req: KakaoPayApproveRequest) -> Result<KakaoPayApproveResponse> {
    let secret_key = env::var("KAKAO_PAY_SECRET_KEY")?;

    let res = client()
        .post("https://open-api.kakaopay.com/online/v1/payment/approve")
        .header("Authorization", format!("SECRET_KEY {}", secret_key))
        .json(&req)
        .send()
        .await?;

    if !res.status().is_success() {
        let text = res.text().await.unwrap_or_default();
        return Err(anyhow!("KakaoPay approve failed: {}", text));
    }

    Ok(res.json().await?)
}

// ── 환불 ───────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct KakaoPayCancelRequest {
    pub cid: String,
    pub tid: String,
    pub cancel_amount: i32,
    pub cancel_tax_free_amount: i32,
}

#[derive(Debug, Deserialize)]
pub struct KakaoPayCancelResponse {
    pub tid: String,
    pub status: String,
    pub canceled_amount: KakaoPayAmount,
    pub canceled_at: String,
}

pub async fn cancel(req: KakaoPayCancelRequest) -> Result<KakaoPayCancelResponse> {
    let secret_key = env::var("KAKAO_PAY_SECRET_KEY")?;

    let res = client()
        .post("https://open-api.kakaopay.com/online/v1/payment/cancel")
        .header("Authorization", format!("SECRET_KEY {}", secret_key))
        .json(&req)
        .send()
        .await?;

    if !res.status().is_success() {
        let text = res.text().await.unwrap_or_default();
        return Err(anyhow!("KakaoPay cancel failed: {}", text));
    }

    Ok(res.json().await?)
}
