CREATE TABLE clients (
    id BIGSERIAL PRIMARY KEY,

    name VARCHAR(64) NOT NULL,
    owner_email VARCHAR(255) NOT NULL,

    key_id VARCHAR(32) NOT NULL UNIQUE,
    secret_hash CHAR(64) NOT NULL,

    scopes TEXT[] NOT NULL DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    expires_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ
);

CREATE INDEX clients_owner_email_idx
    ON clients(owner_email);

CREATE INDEX clients_active_key_idx
    ON clients(key_id)
    WHERE revoked_at IS NULL;
