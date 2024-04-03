-- Add up migration script here
ALTER TABLE answers
ADD COLUMN account_id serial;
