-- Your SQL goes here
ALTER TYPE whatsapp_message_status ADD VALUE 'WHATSAPP_DISCONNECTED';
ALTER TABLE whatsapp_sessions ADD COLUMN is_disconnected boolean NOT NULL DEFAULT false;
