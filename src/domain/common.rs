// src/domain/common.rs
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, EnumString, Display)]
pub enum UserRole {
    USER,
    ADMIN,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, EnumString, Display)]
pub enum SocialProvider {
    NAVER,
    KAKAO,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, EnumString, Display)]
pub enum PaymentProvider {
    NAVER_PAY,
    KAKAO_PAY,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, EnumString, Display)]
pub enum PaymentType {
    INSTANT,
    ONSITE,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, EnumString, Display)]
pub enum BookingStatus {
    PENDING,
    CONFIRMED,
    COMPLETED,
    CANCELLED_BY_USER,
    CANCELLED_BY_ADMIN,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, EnumString, Display)]
pub enum PaymentStatus {
    READY,
    PAID,
    CANCELLED,
    PARTIAL_CANCELLED,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, EnumString, Display)]
pub enum ClosedDayType {
    REGULAR,
    TEMPORARY,
}
