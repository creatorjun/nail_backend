// src/infrastructure/social/mod.rs
pub mod kakao;
pub mod naver;

#[derive(Debug, Clone)]
pub struct SocialProfile {
    pub provider_id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub profile_image_url: Option<String>,
}
