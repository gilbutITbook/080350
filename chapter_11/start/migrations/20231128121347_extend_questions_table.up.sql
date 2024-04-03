-- Add up migration script here
ALTER TABLE questions
ADD COLUMN account_id serial;
