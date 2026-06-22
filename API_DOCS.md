# Nail Salon Backend API 문서

> Base URL: `http://localhost:8080` (개발) / `https://api.yourdomain.com` (운영)

---

## 인증 방식

인증이 필요한 API는 요청 헤더에 Access Token을 포함해야 합니다.

```
Authorization: Bearer {access_token}
```

- **Access Token** 유효기간: 15분
- **Refresh Token** 유효기간: 30일
- Access Token 만료 시 `/auth/refresh`로 재발급

---

## 공통 에러 코드

| 상태코드 | 의미 |
|---|---|
| `400` | 잘못된 요청 (파라미터 오류) |
| `401` | 인증 필요 (토큰 없음 또는 만료) |
| `403` | 권한 없음 (관리자 전용 API) |
| `404` | 리소스 없음 |
| `409` | 충돌 (이미 예약된 시간, 중복 소셜 계정 등) |
| `502` | PG사 API 오류 |
| `500` | 서버 내부 오류 |

---

## 1. 시스템

### `GET /health`
서버 상태 확인

**Response** `200 OK`
```
OK
```

---

## 2. 인증 (Auth)

### `GET /auth/naver`
네이버 OAuth 로그인 URL 반환

**Response** `200 OK`
```json
{
  "url": "https://nid.naver.com/oauth2.0/authorize?..."
}
```

---

### `GET /auth/naver/callback`
네이버 OAuth 콜백 (브라우저가 자동 호출, 앱에서 직접 호출 불필요)

**Query Parameters**
| 파라미터 | 타입 | 설명 |
|---|---|---|
| `code` | string | 네이버 인증 코드 |
| `state` | string | CSRF 방지 토큰 |

**Response** `200 OK`
```json
{
  "access_token": "eyJ...",
  "refresh_token": "eyJ...",
  "token_type": "Bearer"
}
```

---

### `GET /auth/kakao`
카카오 OAuth 로그인 URL 반환

**Response** `200 OK`
```json
{
  "url": "https://kauth.kakao.com/oauth/authorize?..."
}
```

---

### `GET /auth/kakao/callback`
카카오 OAuth 콜백 (브라우저가 자동 호출, 앱에서 직접 호출 불필요)

**Query Parameters**
| 파라미터 | 타입 | 설명 |
|---|---|---|
| `code` | string | 카카오 인증 코드 |

**Response** `200 OK`
```json
{
  "access_token": "eyJ...",
  "refresh_token": "eyJ...",
  "token_type": "Bearer"
}
```

---

### `POST /auth/refresh`
Access Token 재발급

**Request Body**
```json
{
  "refresh_token": "eyJ..."
}
```

**Response** `200 OK`
```json
{
  "access_token": "eyJ...",
  "refresh_token": "eyJ...",
  "token_type": "Bearer"
}
```

---

### `POST /auth/logout`
로그아웃 (Refresh Token 무효화)

**Request Body**
```json
{
  "refresh_token": "eyJ..."
}
```

**Response** `204 No Content`

---

## 3. 유저 (Users)

> 🔒 모든 엔드포인트에 `Authorization` 헤더 필요

### `GET /api/users/me`
내 정보 조회

**Response** `200 OK`
```json
{
  "id": "uuid",
  "name": "홍길동",
  "email": "test@test.com",
  "phone": "010-1234-5678",
  "profile_image_url": "https://...",
  "role": "USER",
  "linked_providers": ["NAVER", "KAKAO"]
}
```

---

### `GET /api/users/link/naver`
현재 계정에 네이버 소셜 계정 추가 연동

**Query Parameters**
| 파라미터 | 타입 | 설명 |
|---|---|---|
| `code` | string | 네이버 인증 코드 |
| `state` | string | CSRF 토큰 |

**Response** `204 No Content`

**Error** `409 Conflict` — 이미 다른 계정에 연동된 소셜 ID

---

### `GET /api/users/link/kakao`
현재 계정에 카카오 소셜 계정 추가 연동

**Query Parameters**
| 파라미터 | 타입 | 설명 |
|---|---|---|
| `code` | string | 카카오 인증 코드 |

**Response** `204 No Content`

**Error** `409 Conflict` — 이미 다른 계정에 연동된 소셜 ID

---

## 4. 서비스 (Services)

### `GET /api/categories`
서비스 카테고리 목록 조회

**Response** `200 OK`
```json
[
  {
    "id": "uuid",
    "name": "젤네일",
    "display_order": 1,
    "is_active": true,
    "created_at": "2026-01-01T00:00:00Z"
  }
]
```

---

### `GET /api/services`
서비스 목록 조회

**Query Parameters** (선택)
| 파라미터 | 타입 | 설명 |
|---|---|---|
| `category_id` | uuid | 카테고리 필터 |

**Response** `200 OK`
```json
[
  {
    "id": "uuid",
    "category_id": "uuid",
    "name": "기본 젤네일",
    "description": "설명",
    "duration_minutes": 60,
    "price_krw": 50000,
    "thumbnail_url": "https://...",
    "display_order": 1,
    "is_active": true,
    "created_at": "2026-01-01T00:00:00Z",
    "updated_at": "2026-01-01T00:00:00Z"
  }
]
```

---

### `GET /api/services/:id`
서비스 단건 조회

**Response** `200 OK` — 위 서비스 객체 단건

**Error** `404 Not Found`

---

## 5. 예약 (Bookings)

### `GET /api/bookings/available-slots`
예약 가능 슬롯 조회 (비로그인 가능)

**Query Parameters**
| 파라미터 | 타입 | 설명 |
|---|---|---|
| `date` | string | 날짜 (YYYY-MM-DD) |
| `service_id` | uuid | 서비스 ID |

**Response** `200 OK`
```json
{
  "date": "2026-06-25",
  "slots": [
    {
      "start": "2026-06-25T01:00:00Z",
      "end": "2026-06-25T02:00:00Z",
      "available": true
    },
    {
      "start": "2026-06-25T02:00:00Z",
      "end": "2026-06-25T03:00:00Z",
      "available": false
    }
  ]
}
```

> ⚠️ 시간은 UTC 기준입니다. 한국 시간(KST)은 +9시간.

---

### `POST /api/bookings` 🔒
예약 생성

**Request Body**
```json
{
  "service_id": "uuid",
  "scheduled_at": "2026-06-25T01:00:00Z",
  "payment_type": "KAKAO_PAY",
  "memo": "오른손만 해주세요"
}
```

| 필드 | 타입 | 필수 | 설명 |
|---|---|---|---|
| `service_id` | uuid | ✅ | 서비스 ID |
| `scheduled_at` | datetime | ✅ | 예약 시작 시간 (UTC ISO8601) |
| `payment_type` | string | ✅ | `NAVER_PAY` \| `KAKAO_PAY` \| `ONSITE` |
| `memo` | string | ❌ | 고객 요청사항 |

**Response** `201 Created`
```json
{
  "id": "uuid",
  "user_id": "uuid",
  "service_id": "uuid",
  "scheduled_at": "2026-06-25T01:00:00Z",
  "end_at": "2026-06-25T02:00:00Z",
  "payment_type": "KAKAO_PAY",
  "status": "PENDING",
  "memo": "오른손만 해주세요",
  "admin_memo": null,
  "created_at": "2026-06-22T00:00:00Z",
  "updated_at": "2026-06-22T00:00:00Z"
}
```

**Booking Status 값**
| 값 | 의미 |
|---|---|
| `PENDING` | 결제 대기 (NAVER_PAY, KAKAO_PAY) |
| `CONFIRMED` | 예약 확정 (결제 완료 또는 ONSITE) |
| `COMPLETED` | 시술 완료 |
| `CANCELLED_BY_USER` | 고객 취소 |
| `CANCELLED_BY_ADMIN` | 관리자 취소 |

**Error** `409 Conflict` — 이미 예약된 시간대

---

### `GET /api/bookings/my` 🔒
내 예약 목록 조회

**Response** `200 OK` — 예약 객체 배열 (최신순)

---

### `GET /api/bookings/:id` 🔒
예약 단건 조회

**Response** `200 OK` — 예약 객체 단건

**Error** `404 Not Found`

---

### `POST /api/bookings/:id/cancel` 🔒
예약 취소 (본인 예약만 가능, 완료/취소 상태는 불가)

**Response** `200 OK` — 취소된 예약 객체

**Error** `404 Not Found` — 취소 불가 상태이거나 본인 예약이 아님

---

## 6. 결제 (Payments)

### `POST /api/payments/naver/ready` 🔒
네이버페이 결제 준비

**Request Body**
```json
{
  "booking_id": "uuid",
  "product_name": "기본 젤네일",
  "amount": 50000,
  "return_url": "http://localhost:8080/api/payments/naver/approve"
}
```

**Response** `200 OK`
```json
{
  "pg_order_id": "NAVER-uuid",
  "payment_url": "https://pay.naver.com/..."
}
```

> Flutter에서 `payment_url`을 WebView로 열면 됩니다.

---

### `GET /api/payments/naver/approve`
네이버페이 결제 승인 콜백 (PG사 → 서버 자동 호출, 앱에서 직접 호출 불필요)

**Query Parameters** (네이버페이 자동 전달)
| 파라미터 | 타입 | 설명 |
|---|---|---|
| `pg_order_id` | string | 주문번호 |
| `merchant_user_key` | string | 유저 식별키 |
| `payment_id` | string | 네이버페이 결제 ID |

**Response** `200 OK` — Payment 객체

---

### `POST /api/payments/kakao/ready` 🔒
카카오페이 결제 준비

**Request Body**
```json
{
  "booking_id": "uuid",
  "product_name": "기본 젤네일",
  "amount": 50000
}
```

**Response** `200 OK`
```json
{
  "pg_order_id": "KAKAO-uuid",
  "payment_url": "https://online-pay.kakao.com/..."
}
```

> Flutter에서 `payment_url`을 WebView로 열면 됩니다.

---

### `GET /api/payments/kakao/approve`
카카오페이 결제 승인 콜백 (PG사 → 서버 자동 호출, 앱에서 직접 호출 불필요)

**Query Parameters** (카카오페이 자동 전달)
| 파라미터 | 타입 | 설명 |
|---|---|---|
| `pg_token` | string | 결제 승인 토큰 |
| `partner_order_id` | string | 주문번호 |

**Response** `200 OK` — Payment 객체

---

## 7. 관리자 (Admin)

> 🔒🔑 모든 엔드포인트에 `Authorization` 헤더 필요 + `role = ADMIN`

### 카테고리 관리

| Method | URL | 설명 |
|---|---|---|
| `GET` | `/api/admin/categories` | 카테고리 전체 조회 |
| `POST` | `/api/admin/categories` | 카테고리 생성 |
| `PUT` | `/api/admin/categories/:id` | 카테고리 수정 |
| `DELETE` | `/api/admin/categories/:id` | 카테고리 삭제 |

**POST / PUT Request Body**
```json
{
  "name": "젤네일",
  "display_order": 1,
  "is_active": true
}
```

---

### 서비스 관리

| Method | URL | 설명 |
|---|---|---|
| `GET` | `/api/admin/services` | 서비스 전체 조회 (비활성 포함) |
| `POST` | `/api/admin/services` | 서비스 생성 |
| `PUT` | `/api/admin/services/:id` | 서비스 수정 |
| `DELETE` | `/api/admin/services/:id` | 서비스 비활성화 |

**POST / PUT Request Body**
```json
{
  "category_id": "uuid",
  "name": "기본 젤네일",
  "description": "설명",
  "duration_minutes": 60,
  "price_krw": 50000,
  "thumbnail_url": "https://...",
  "display_order": 1,
  "is_active": true
}
```

---

### 예약 관리

| Method | URL | 설명 |
|---|---|---|
| `GET` | `/api/admin/bookings` | 전체 예약 조회 |
| `PUT` | `/api/admin/bookings/:id/status` | 예약 상태 변경 |
| `POST` | `/api/admin/bookings/:id/refund` | 환불 처리 (PG사 API 실제 호출) |

**PUT status Request Body**
```json
{
  "status": "COMPLETED",
  "admin_memo": "시술 완료"
}
```

**POST refund Request Body**
```json
{
  "refund_amount_krw": 50000,
  "refund_reason": "고객 요청 취소"
}
```

---

### 휴무일 관리

| Method | URL | 설명 |
|---|---|---|
| `GET` | `/api/admin/closed-days` | 휴무일 목록 조회 |
| `POST` | `/api/admin/closed-days` | 휴무일 추가 |
| `DELETE` | `/api/admin/closed-days/:id` | 휴무일 삭제 |

**POST Request Body**
```json
{
  "closed_date": "2026-12-25",
  "type": "HOLIDAY",
  "reason": "크리스마스"
}
```

`type` 값: `HOLIDAY` | `TEMPORARY` | `REGULAR`

---

### 샵 설정

| Method | URL | 설명 |
|---|---|---|
| `GET` | `/api/admin/shop-settings` | 샵 설정 조회 |
| `PUT` | `/api/admin/shop-settings` | 샵 설정 수정 |

**GET Response / PUT Request Body**
```json
{
  "shop_name": "홍길동 네일샵",
  "closed_weekdays": [0, 1],
  "open_time": "10:00",
  "close_time": "20:00",
  "slot_interval_min": 30,
  "max_booking_days": 30
}
```

`closed_weekdays` 값: `0`=일요일, `1`=월요일, ..., `6`=토요일

---

## 8. Flutter 연동 흐름

### 소셜 로그인
```
1. GET /auth/naver  →  url 받기
2. WebView로 url 열기
3. 콜백으로 access_token, refresh_token 수신
4. 로컬 저장 후 이후 요청에 Authorization 헤더 첨부
```

### 예약 + 결제
```
1. GET /api/bookings/available-slots  →  슬롯 선택
2. POST /api/bookings  →  booking_id 획득, status: PENDING
3. POST /api/payments/kakao/ready  →  payment_url 획득
4. WebView로 payment_url 열기
5. 결제 완료 후 approve 콜백 자동 처리
6. booking status CONFIRMED 확인
```

### Token 갱신
```
1. API 요청 시 401 응답 받으면
2. POST /auth/refresh  →  새 access_token 발급
3. 실패 시 로그아웃 처리 후 로그인 화면으로
```
