-- Your SQL goes here
CREATE TABLE api_keys (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    user_id uuid NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    api_key VARCHAR(255) NOT NULL
);

SELECT
    diesel_manage_updated_at ('api_keys');
