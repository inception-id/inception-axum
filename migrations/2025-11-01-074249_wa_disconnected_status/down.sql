-- This file should undo anything in `up.sql`
ALTER TABLE whatsapp_sessions ADD COLUMN is_disconnected boolean NOT NULL DEFAULT false;
