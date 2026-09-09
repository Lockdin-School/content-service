-- Add migration script here
-- Remove the strict one-attempt-per-student-per-quiz constraint
ALTER TABLE quiz_attempts
    DROP CONSTRAINT IF EXISTS quiz_attempts_quiz_id_student_id_unique;

-- Allow multiple completed/abandoned attempts,
-- but only one in-progress attempt per student per quiz.
CREATE UNIQUE INDEX IF NOT EXISTS quiz_attempts_one_in_progress_per_student_quiz
    ON quiz_attempts (quiz_id, student_id)
    WHERE status = 'inprogress';