-- Your SQL goes here
ALTER TABLE whatsapp_sessions ADD COLUMN hourly_limit INTEGER NOT NULL DEFAULT 100;
ALTER TABLE whatsapp_sessions ADD COLUMN daily_limit INTEGER NOT NULL DEFAULT 1000;
