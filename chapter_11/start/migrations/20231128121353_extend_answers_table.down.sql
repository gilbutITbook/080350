-- Add down migration script here
ALTER TABLE answers
DROP COLUMN account_id;
