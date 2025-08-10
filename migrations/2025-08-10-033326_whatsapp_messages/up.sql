-- Your SQL goes here

CREATE TYPE whatsapp_message_type AS ENUM ('DEVELOPMENT', 'PRODUCTION');

CREATE TABLE whatsapp_messages (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    session_id uuid NOT NULL REFERENCES whatsapp_sessions(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    target_phone VARCHAR(255) NOT NULL DEFAULT '',
    message_type whatsapp_message_type NOT NULL DEFAULT 'DEVELOPMENT',
    text_message TEXT
);

SELECT
    diesel_manage_updated_at ('whatsapp_messages');
