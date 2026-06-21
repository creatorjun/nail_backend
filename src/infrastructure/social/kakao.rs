// src/infrastructure/social/kakao.rs
use super::SocialProfile;
use anyhow::Result;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct KakaoTokenResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct KakaoProfileResponse {
    id: i64,
    kakao_account: Option<KakaoAccount>,
}

#[derive(Debug, Deserialize)]
struct KakaoAccount {
    email: Option<String>,
    profile: Option<KakaoProfile>,
    phone_number: Option<String>,
}

#[derive(Debug, Deserialize)]
struct KakaoProfile {
    nickname: Option<String>,
    profile_image_url: Option<String>,
}

pub fn get_oauth_url(state: &str) -> String {
    let client_id = env::var("KAKAO_CLIENT_ID").expect("KAKAO_CLIENT_ID must be set");
    let redirect_uri = env::var("KAKAO_REDIRECT_URI").expect("KAKAO_REDIRECT_URI must be set");
    format!(
        "https://kauth.kakao.com/oauth/authorize?response_type=code&client_id={}&redirect_uri={}&state={}",
        client_id, redirect_uri, state
    )
}

pub async fn fetch_profile(code: &str) -> Result<SocialProfile> {
    let client_id = env::var("KAKAO_CLIENT_ID")?;
    let client_secret = env::var("KAKAO_CLIENT_SECRET")?;
    let redirect_uri = env::var("KAKAO_REDIRECT_URI")?;

    let client = reqwest::Client::new();

    let token_res: KakaoTokenResponse = client
        .post("https://kauth.kakao.com/oauth/token")
        .form(&[
            ("grant_type", "authorization_code"),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
            ("redirect_uri", &redirect_uri),
            ("code", code),
        ])
        .send()
        .await?
        .json()
        .await?;

    let profile_res: KakaoProfileResponse = client
        .get("https://kapi.kakao.com/v2/user/me")
        .bearer_auth(&token_res.access_token)
        .send()
        .await?
        .json()
        .await?;

    let account = profile_res.kakao_account.unwrap_or(KakaoAccount {
        email: None,
        profile: None,
        phone_number: None,
    });
    let profile = account.profile.unwrap_or(KakaoProfile {
        nickname: None,
        profile_image_url: None,
    });

    Ok(SocialProfile {
        provider_id: profile_res.id.to_string(),
        name: profile.nickname.unwrap_or_else(|| "이름없음".into()),
        email: account.email,
        phone: account.phone_number,
        profile_image_url: profile.profile_image_url,
    })
}
