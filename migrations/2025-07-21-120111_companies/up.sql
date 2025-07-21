-- Your SQL goes here

CREATE TABLE companies (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    name VARCHAR NOT NULL,
    phone VARCHAR(255) NOT NULL,
    api_key VARCHAR(255) NOT NULL
);

SELECT
    diesel_manage_updated_at ('companies');

CREATE TYPE companies_users_permission AS ENUM ('owner', 'edit', 'view');

CREATE TABLE companies_users (
    company_id uuid REFERENCES companies (id) ON UPDATE CASCADE ON DELETE CASCADE,
    user_id uuid REFERENCES users (id) ON UPDATE CASCADE ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    permission companies_users_permission NOT NULL DEFAULT 'view',
    CONSTRAINT companies_users_pkey PRIMARY KEY (company_id, user_id)
);

SELECT
    diesel_manage_updated_at ('companies_users');
