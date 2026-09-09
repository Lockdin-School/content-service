-- Add migration script here
ALTER TABLE quiz_attempts
    ADD CONSTRAINT quiz_attempts_quiz_id_student_id_unique
        UNIQUE (quiz_id, student_id);