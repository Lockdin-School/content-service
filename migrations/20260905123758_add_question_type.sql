-- Add migration script here
ALTER TABLE quizzes DROP COLUMN question_type;
ALTER TABLE quiz_questions ADD COLUMN question_type question_type;