-- This file should undo anything in `up.sql`
CREATE TYPE storage_visibility AS ENUM ('public', 'private');
ALTER TABLE translation_storage ADD COLUMN visibility storage_visibility NOT NULL DEFAULT 'private';
ALTER TABLE checkbot_storage ADD COLUMN visibility storage_visibility NOT NULL DEFAULT 'private';
ALTER TABLE speech_to_text_storage ADD COLUMN visibility storage_visibility NOT NULL DEFAULT 'private';
ALTER TABLE text_to_speech_storage ADD COLUMN visibility storage_visibility NOT NULL DEFAULT 'private';

DROP TABLE shared_translation_storage;
DROP TYPE shared_storage_permission;
