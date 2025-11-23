-- This file should undo anything in `up.sql`

ALTER TABLE whatsapp_messages DROP COLUMN direction;

DROP TYPE IF EXISTS whatsapp_message_direction;

ALTER TABLE whatsapp_messages DROP COLUMN user_id;
