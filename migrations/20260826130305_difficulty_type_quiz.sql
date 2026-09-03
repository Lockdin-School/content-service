-- Add migration script here
CREATE TYPE difficulty AS ENUM (
    'easy',
    'medium',
    'hard'
    );

ALTER TABLE quizzes DROP COLUMN difficulty;
ALTER TABLE quizzes ADD COLUMN difficulty difficulty;