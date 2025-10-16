-- Your SQL goes here
ALTER TABLE whatsapp_payments ADD COLUMN doku_notif JSONB;
ALTER TABLE whatsapp_payments ADD COLUMN per_unit_price NUMERIC NOT NULL DEFAULT 0;
