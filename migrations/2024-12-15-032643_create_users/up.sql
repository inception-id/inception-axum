-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    supertokens_user_id uuid,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(255)
);

SELECT
    diesel_manage_updated_at ('users');
