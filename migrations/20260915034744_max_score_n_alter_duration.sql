-- Add migration script here
ALTER TABLE quizzes
    ALTER COLUMN estimated_duration_minutes TYPE BIGINT,
    ADD COLUMN max_score INTEGER NOT NULL;

ALTER TABLE quizzes
    RENAME COLUMN estimated_duration_minutes TO estimated_duration_seconds;
