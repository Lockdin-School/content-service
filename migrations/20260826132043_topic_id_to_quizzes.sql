-- Add migration script here
ALTER TABLE quizzes ADD COLUMN topic_id UUID NOT NULL;