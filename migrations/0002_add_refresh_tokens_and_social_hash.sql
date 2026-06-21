ALTER TABLE user_social_accounts
ADD COLUMN provider_id_hash TEXT NOT NULL DEFAULT '';

CREATE UNIQUE INDEX idx_social_provider_hash
    ON user_social_accounts(provider, provider_id_hash);

CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_refresh_tokens_user_id ON refresh_tokens(user_id);
