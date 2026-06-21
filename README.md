# nail-salon-backend

Axum + sqlx + PostgreSQL 기반 네일샵 예약 백엔드

## 기술 스택

- Axum 0.8
- PostgreSQL + sqlx 0.8
- Tokio
- JWT
- AES-256-GCM (소셜 식별 ID 암호화)

## 주요 기능

- 네이버 / 카카오 소셜 로그인 연동 구조
- 네이버+카카오 동시 연동 지원
- 사용자 / 관리자 권한 분리
- 시술 카테고리 / 시술 관리
- 예약 생성 / 조회 / 취소
- 즉시 결제 / 현장 결제 구조 지원
- 고정 휴무 / 임시 휴무 관리
- 예약 가능 슬롯 조회
- PostgreSQL EXCLUDE 제약 기반 더블 부킹 방지

## 프로젝트 구조

```
src/
├── domain/
├── application/
├── infrastructure/
└── presentation/
```

## 시작하기

```bash
cp .env.example .env
sqlx migrate run
cargo run
```

## 기본 엔드포인트

- GET /health
- GET /api/categories
- GET /api/services
- GET /api/bookings/available-slots?date=2026-06-25&service_id=UUID
- GET /api/bookings/my
- POST /api/bookings
