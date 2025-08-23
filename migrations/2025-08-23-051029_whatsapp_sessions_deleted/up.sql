-- Your SQL goes here
ALTER TABLE whatsapp_sessions ADD COLUMN is_deleted BOOLEAN NOT NULL DEFAULT false;
