-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    supertokens_user_id uuid,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(255),
    is_super_admin BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE companies (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    name VARCHAR(255) NOT NULL,
    legal_name VARCHAR(255),
    email VARCHAR(255),
    phone VARCHAR(255),
    website VARCHAR(255),
    api_key VARCHAR(255),
    ip_address JSONB NOT NULL DEFAULT '[]'
);

CREATE TABLE company_users (
    user_id uuid NOT NULL REFERENCES users (id),
    company_id uuid NOT NULL REFERENCES companies (id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    CONSTRAINT company_users_pk PRIMARY KEY (user_id, company_id)
);

SELECT
    diesel_manage_updated_at ('users');

SELECT
    diesel_manage_updated_at ('companies');

SELECT
    diesel_manage_updated_at ('company_users');
