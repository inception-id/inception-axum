-- Your SQL goes here
ALTER TABLE whatsapp_payments ADD COLUMN doku_notif JSONB NOT NULL DEFAULT '{}';
