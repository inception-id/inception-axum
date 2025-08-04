-- Your SQL goes here
CREATE TABLE whatsapp_sessions (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    user_id uuid NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    phone VARCHAR(255) NOT NULL
);

SELECT
    diesel_manage_updated_at ('whatsapp_sessions');
