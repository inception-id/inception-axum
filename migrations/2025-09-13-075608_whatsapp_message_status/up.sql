-- Your SQL goes here
CREATE TYPE whatsapp_message_status AS ENUM ('PENDING', 'DELIVERED');
ALTER TABLE whatsapp_messages ADD COLUMN status whatsapp_message_status NOT NULL DEFAULT 'DELIVERED';
ALTER TABLE whatsapp_notifications ADD COLUMN status whatsapp_message_status NOT NULL DEFAULT 'DELIVERED';
