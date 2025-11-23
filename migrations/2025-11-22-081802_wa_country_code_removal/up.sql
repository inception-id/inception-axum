-- Your SQL goes here

ALTER TABLE whatsapp_messages ADD COLUMN user_id uuid REFERENCES users(id);

CREATE TYPE whatsapp_message_direction AS ENUM ('INCOMING', 'OUTGOING');

ALTER TABLE whatsapp_messages ADD COLUMN direction whatsapp_message_direction NOT NULL DEFAULT 'OUTGOING';
