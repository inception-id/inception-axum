-- This file should undo anything in `up.sql`
ALTER TABLE whatsapp_messages DROP COLUMN environment ;
CREATE TYPE whatsapp_message_type AS ENUM ('DEVELOPMENT', 'PRODUCTION');
ALTER TABLE whatsapp_messages ADD COLUMN message_type whatsapp_message_type NOT NULL DEFAULT 'DEVELOPMENT'
