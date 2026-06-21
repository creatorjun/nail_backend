CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS btree_gist;

CREATE TABLE service_categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL UNIQUE,
    display_order INT NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE services (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    category_id UUID REFERENCES service_categories(id),
    name TEXT NOT NULL,
    description TEXT,
    duration_minutes INT NOT NULL,
    price_krw INT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    display_order INT NOT NULL DEFAULT 0,
    thumbnail_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    profile_image_url TEXT,
    role TEXT NOT NULL DEFAULT 'USER',
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_social_accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    provider_id_encrypted TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (provider, provider_id_encrypted)
);

CREATE TABLE shop_settings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    shop_name TEXT NOT NULL DEFAULT '네일샵',
    closed_weekdays INT[] NOT NULL DEFAULT '{}',
    open_time TIME NOT NULL DEFAULT '10:00',
    close_time TIME NOT NULL DEFAULT '20:00',
    slot_interval_min INT NOT NULL DEFAULT 30,
    max_booking_days INT NOT NULL DEFAULT 30,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE closed_days (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    closed_date DATE NOT NULL UNIQUE,
    type TEXT NOT NULL DEFAULT 'TEMPORARY',
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE bookings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    service_id UUID NOT NULL REFERENCES services(id),
    scheduled_at TIMESTAMPTZ NOT NULL,
    end_at TIMESTAMPTZ NOT NULL,
    payment_type TEXT NOT NULL DEFAULT 'INSTANT',
    status TEXT NOT NULL DEFAULT 'PENDING',
    memo TEXT,
    admin_memo TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT bookings_valid_time CHECK (end_at > scheduled_at)
);

ALTER TABLE bookings
ADD CONSTRAINT no_double_booking
EXCLUDE USING gist (
    tstzrange(scheduled_at, end_at, '[)') WITH &&
)
WHERE (status NOT IN ('CANCELLED_BY_USER', 'CANCELLED_BY_ADMIN'));

CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    booking_id UUID NOT NULL REFERENCES bookings(id),
    user_id UUID NOT NULL REFERENCES users(id),
    provider TEXT NOT NULL,
    payment_type TEXT NOT NULL,
    amount_krw INT NOT NULL,
    status TEXT NOT NULL DEFAULT 'READY',
    pg_order_id TEXT NOT NULL UNIQUE,
    pg_transaction_id TEXT UNIQUE,
    paid_at TIMESTAMPTZ,
    refund_amount_krw INT,
    refund_reason TEXT,
    refunded_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_services_category_id ON services(category_id);
CREATE INDEX idx_bookings_scheduled_at ON bookings(scheduled_at);
CREATE INDEX idx_bookings_user_id ON bookings(user_id);
CREATE INDEX idx_bookings_status ON bookings(status);
CREATE INDEX idx_payments_booking_id ON payments(booking_id);
CREATE INDEX idx_user_social_accounts_user_id ON user_social_accounts(user_id);

INSERT INTO shop_settings DEFAULT VALUES;
