-- This file should undo anything in `up.sql`
ALTER TABLE whatsapp_payments DROP COLUMN paid_at;
ALTER TABLE whatsapp_payments DROP COLUMN year;
ALTER TABLE whatsapp_payments DROP COLUMN month;
