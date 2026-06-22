// src/infrastructure/payment/naver_pay.rs
//
// 네이버페이 Orders API v2
// 문서: https://developer.pay.naver.com/docs/v2/api

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::env;

fn base_url() -> String {
    match env::var("NAVER_PAY_ENV").as_deref() {
        Ok("production") => "https://pay.naver.com".into(),
        _ => "https://sandbox-pay.naver.com".into(),
    }
}

fn client() -> reqwest::Client {
    reqwest::Client::new()
}

// ── 결제 준비 ──────────────────────────────────────────────────

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NaverPayReadyRequest {
    pub merchant_user_key: String,
    pub merchant_pay_key: String,
    pub product_name: String,
    pub total_pay_amount: i32,
    pub tax_scope_amount: i32,
    pub tax_ex_scope_amount: i32,
    pub return_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NaverPayReadyResponse {
    pub result_code: String,
    pub result_message: String,
    pub pay_highlight_url: Option<String>,
    pub pay_express_url: Option<String>,
}

pub async fn ready(req: NaverPayReadyRequest) -> Result<NaverPayReadyResponse> {
    let client_id = env::var("NAVER_PAY_CLIENT_ID")?;
    let client_secret = env::var("NAVER_PAY_CLIENT_SECRET")?;
    let chain_id = env::var("NAVER_PAY_CHAIN_ID")?;
    let url = format!("{}/v2/payments/ready", base_url());

    let res = client()
        .post(&url)
        .header("X-Naver-Client-Id", &client_id)
        .header("X-Naver-Client-Secret", &client_secret)
        .header("X-NaverPay-Chain-Id", &chain_id)
        .json(&req)
        .send()
        .await?;

    let body: NaverPayReadyResponse = res.json().await?;
    if body.result_code != "Success" {
        return Err(anyhow!("NaverPay ready failed: {}", body.result_message));
    }
    Ok(body)
}

// ── 결제 승인 ──────────────────────────────────────────────────

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NaverPayApproveRequest {
    pub merchant_pay_key: String,
    pub merchant_user_key: String,
    pub payment_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NaverPayApproveDetail {
    pub payment_id: String,
    pub detail: Option<NaverPayDetail>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NaverPayDetail {
    pub payment_id: String,
    pub primary_pay_amount: i32,
    pub merchant_pay_key: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NaverPayApproveResponse {
    pub result_code: String,
    pub result_message: String,
    pub body: Option<NaverPayApproveDetail>,
}

pub async fn approve(req: NaverPayApproveRequest) -> Result<NaverPayApproveResponse> {
    let client_id = env::var("NAVER_PAY_CLIENT_ID")?;
    let client_secret = env::var("NAVER_PAY_CLIENT_SECRET")?;
    let chain_id = env::var("NAVER_PAY_CHAIN_ID")?;
    let url = format!("{}/v2/payments/apply", base_url());

    let res = client()
        .post(&url)
        .header("X-Naver-Client-Id", &client_id)
        .header("X-Naver-Client-Secret", &client_secret)
        .header("X-NaverPay-Chain-Id", &chain_id)
        .json(&req)
        .send()
        .await?;

    let body: NaverPayApproveResponse = res.json().await?;
    if body.result_code != "Success" {
        return Err(anyhow!("NaverPay approve failed: {}", body.result_message));
    }
    Ok(body)
}

// ── 환불 ───────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NaverPayCancelRequest {
    pub payment_id: String,
    pub cancel_amount: i32,
    pub cancel_reason: String,
    pub cancel_require_point: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NaverPayCancelResponse {
    pub result_code: String,
    pub result_message: String,
}

pub async fn cancel(req: NaverPayCancelRequest) -> Result<NaverPayCancelResponse> {
    let client_id = env::var("NAVER_PAY_CLIENT_ID")?;
    let client_secret = env::var("NAVER_PAY_CLIENT_SECRET")?;
    let chain_id = env::var("NAVER_PAY_CHAIN_ID")?;
    let url = format!("{}/v2/payments/cancel", base_url());

    let res = client()
        .post(&url)
        .header("X-Naver-Client-Id", &client_id)
        .header("X-Naver-Client-Secret", &client_secret)
        .header("X-NaverPay-Chain-Id", &chain_id)
        .json(&req)
        .send()
        .await?;

    let body: NaverPayCancelResponse = res.json().await?;
    if body.result_code != "Success" {
        return Err(anyhow!("NaverPay cancel failed: {}", body.result_message));
    }
    Ok(body)
}
