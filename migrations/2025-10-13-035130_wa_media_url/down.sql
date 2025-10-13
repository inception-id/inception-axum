-- This file should undo anything in `up.sql`

ALTER TABLE whatsapp_notifications DROP COLUMN media_url;
ALTER TABLE whatsapp_messages DROP COLUMN media_url;
