-- Your SQL goes here
CREATE TYPE whatsapp_payment_status AS ENUM ('FAIL', 'PAID', 'PENDING');

CREATE TABLE whatsapp_payments (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    user_id uuid NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    payment_status whatsapp_payment_status NOT NULL DEFAULT 'PENDING',
    amount numeric NOT NULL DEFAULT 0,
    items jsonb NOT NULL DEFAULT '[]',
    doku_request jsonb,
    doku_response jsonb
);

SELECT
    diesel_manage_updated_at ('whatsapp_payments');
