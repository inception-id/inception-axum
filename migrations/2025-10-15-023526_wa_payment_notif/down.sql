-- This file should undo anything in `up.sql`
ALTER TABLE whatsapp_payments DROP COLUMN doku_notif;
ALTER TABLE whatsapp_payments DROP COLUMN per_unit_price;
