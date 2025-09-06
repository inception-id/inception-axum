-- This file should undo anything in `up.sql`
ALTER TABLE whatsapp_sessions DROP COLUMN hourly_limit;
ALTER TABLE whatsapp_sessions DROP COLUMN daily_limit;
