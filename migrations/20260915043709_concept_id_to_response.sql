-- Add migration script here
ALTER TABLE quiz_question_responses ADD COLUMN concept_id uuid not null;