-- Add migration script here
CREATE TYPE question_type AS ENUM (
    'multipleChoice'
    );

ALTER TABLE quiz_questions DROP COLUMN question_type;
ALTER TABLE quizzes ADD COLUMN question_type question_type;