-- Your SQL goes here
ALTER TABLE whatsapp_messages DROP COLUMN message_type;
DROP TYPE whatsapp_message_type;
ALTER TABLE whatsapp_messages ADD COLUMN environment whatsapp_environment NOT NULL DEFAULT 'DEVELOPMENT'
