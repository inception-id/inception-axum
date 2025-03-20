-- Your SQL goes here
CREATE TABLE exchange_developers (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    user_id uuid NOT NULL REFERENCES users (id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    api_key VARCHAR(255) NOT NULL,
    api_cost FLOAT NOT NULL DEFAULT 10.0,
    balance FLOAT NOT NULL DEFAULT 0.0
);
