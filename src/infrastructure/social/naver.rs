// src/infrastructure/social/naver.rs
use super::SocialProfile;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct NaverTokenResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct NaverProfileResponse {
    response: NaverProfileInner,
}

#[derive(Debug, Deserialize)]
struct NaverProfileInner {
    id: String,
    name: Option<String>,
    email: Option<String>,
    mobile: Option<String>,
    profile_image: Option<String>,
}

pub fn get_oauth_url(state: &str) -> String {
    let client_id = env::var("NAVER_CLIENT_ID").expect("NAVER_CLIENT_ID must be set");
    let redirect_uri = env::var("NAVER_REDIRECT_URI").expect("NAVER_REDIRECT_URI must be set");
    format!(
        "https://nid.naver.com/oauth2.0/authorize?response_type=code&client_id={}&redirect_uri={}&state={}",
        client_id, redirect_uri, state
    )
}

pub async fn fetch_profile(code: &str, state: &str) -> Result<SocialProfile> {
    let client_id = env::var("NAVER_CLIENT_ID")?;
    let client_secret = env::var("NAVER_CLIENT_SECRET")?;
    let redirect_uri = env::var("NAVER_REDIRECT_URI")?;

    let client = reqwest::Client::new();

    let token_res: NaverTokenResponse = client
        .post("https://nid.naver.com/oauth2.0/token")
        .query(&[
            ("grant_type", "authorization_code"),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
            ("redirect_uri", &redirect_uri),
            ("code", code),
            ("state", state),
        ])
        .send()
        .await?
        .json()
        .await?;

    let profile_res: NaverProfileResponse = client
        .get("https://openapi.naver.com/v1/nid/me")
        .bearer_auth(&token_res.access_token)
        .send()
        .await?
        .json()
        .await?;

    let inner = profile_res.response;
    Ok(SocialProfile {
        provider_id: inner.id,
        name: inner.name.unwrap_or_else(|| "이름없음".into()),
        email: inner.email,
        phone: inner.mobile,
        profile_image_url: inner.profile_image,
    })
}
