-- This file should undo anything in `up.sql`
ALTER TABLE whatsapp_notifications DROP COLUMN country_code ;
ALTER TABLE whatsapp_messages DROP COLUMN country_code;
