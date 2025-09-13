-- This file should undo anything in `up.sql`
ALTER TABLE whatsapp_messages DROP COLUMN status;
ALTER TABLE whatsapp_notifications DROP COLUMN status;
DROP TYPE whatsapp_message_status;
