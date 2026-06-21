# nail-salon-backend

Axum + sqlx + PostgreSQL + async-stripe 기반 네일샵 예약·결제 백엔드

## 기술 스택

- **Framework**: Axum 0.8
- **Database**: PostgreSQL + sqlx 0.8
- **Payment**: async-stripe 0.40
- **Auth**: JWT (jsonwebtoken)
- **Runtime**: Tokio

## 프로젝트 구조

```
src/
├── domain/          # Entity, 도메인 모델
├── application/     # UseCase, 비즈니스 로직
├── infrastructure/  # DB 구현체, Stripe 클라이언트
└── presentation/    # Axum Router, Handler, DTO
```

## 시작하기

```bash
# 환경변수 설정
cp .env.example .env
# .env 파일 편집

# DB 마이그레이션
sqlx migrate run

# 서버 실행
cargo run
```

## API 엔드포인트

| Method | Path | 설명 |
|---|---|---|
| GET | /health | 헬스체크 |
| GET | /api/services | 서비스 목록 |
| GET | /api/services/:id | 서비스 상세 |
| GET | /api/bookings | 예약 목록 |
| POST | /api/bookings | 예약 생성 |
| GET | /api/bookings/:id | 예약 상세 |
| POST | /api/payments/intent | 결제 Intent 생성 |
```
