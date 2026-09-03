-- Add migration script here
ALTER TABLE quiz_questions
    ADD COLUMN options       JSONB   NOT NULL DEFAULT '[]'::jsonb;

