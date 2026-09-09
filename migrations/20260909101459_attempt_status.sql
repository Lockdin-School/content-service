-- Add migration script here
CREATE TYPE status AS ENUM ('pending', 'inprogress', 'completed');

ALTER TABLE quiz_attempts DROP COLUMN status;
ALTER TABLE quiz_attempts ADD COLUMN status status;