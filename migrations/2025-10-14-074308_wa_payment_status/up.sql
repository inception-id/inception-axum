-- Your SQL goes here
ALTER TYPE whatsapp_payment_status ADD VALUE 'FREE';
ALTER TABLE whatsapp_payments ADD COLUMN paid_at TIMESTAMP;
ALTER TABLE whatsapp_payments ADD COLUMN year INTEGER;
ALTER TABLE whatsapp_payments ADD COLUMN month INTEGER;
