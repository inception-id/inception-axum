-- Your SQL goes here

CREATE TYPE whatsapp_environment AS ENUM ('DEVELOPMENT', 'PRODUCTION');

CREATE TABLE whatsapp_notifications (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    session_id uuid NOT NULL REFERENCES whatsapp_sessions(id) ON DELETE CASCADE,
    user_id uuid NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    target_phone VARCHAR(255) NOT NULL DEFAULT '',
    text_message TEXT,
    environment whatsapp_environment NOT NULL DEFAULT 'DEVELOPMENT'
);

SELECT
    diesel_manage_updated_at ('whatsapp_notifications');
