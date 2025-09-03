-- Your SQL goes here
ALTER TABLE whatsapp_notifications ADD COLUMN country_code VARCHAR(255) NOT NULL DEFAULT '+62';
ALTER TABLE whatsapp_messages ADD COLUMN country_code VARCHAR(255) NOT NULL DEFAULT '+62';
