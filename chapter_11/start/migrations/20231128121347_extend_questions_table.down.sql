-- Add down migration script here
ALTER TABLE questions
DROP COLUMN account_id;
