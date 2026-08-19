-- Add migration script here

-- Stores the definition and configuration of a quiz associated with a lesson.
CREATE TABLE quizzes
(
    id                         UUID PRIMARY KEY,
    lesson_id                  UUID        NOT NULL,

    title                      TEXT        NOT NULL,
    description                TEXT,

    -- Difficulty is represented as a constrained value to keep the
    -- persisted domain model deterministic.
    difficulty                 TEXT        NOT NULL
        CHECK (difficulty IN ('easy', 'medium', 'hard')),

    -- Minimum score required for the quiz to be considered passed.
    passing_score              INTEGER     NOT NULL,

    -- Expected completion time expressed in minutes.
    estimated_duration_minutes INTEGER     NOT NULL,

    created_at                 TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at                 TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- Stores the questions that make up a quiz.
--
-- Questions are owned by their quiz and are therefore removed when
-- the parent quiz is deleted.
CREATE TABLE quiz_questions
(
    id            UUID PRIMARY KEY,
    quiz_id       UUID    NOT NULL REFERENCES quizzes (id) ON DELETE CASCADE,

    -- Identifies the format used to present and evaluate the question.
    question_type TEXT    NOT NULL
        CHECK (question_type IN ('multipleChoice')),

    prompt        TEXT    NOT NULL,

    -- Number of points awarded when the question is answered correctly.
    points        INTEGER NOT NULL DEFAULT 1,

    -- Position of the question within the quiz.
    "order"       INTEGER NOT NULL,

    -- Prevents two questions in the same quiz from occupying the same position.
    UNIQUE (quiz_id, "order")
);


-- Stores the selectable options belonging to a quiz question.
--
-- Options are owned by their question and are removed when the parent
-- question is deleted.
CREATE TABLE quiz_question_options
(
    id          UUID PRIMARY KEY,
    question_id UUID    NOT NULL
        REFERENCES quiz_questions (id)
            ON DELETE CASCADE,

    text        TEXT    NOT NULL,

    -- Identifies whether this option represents a correct answer.
    -- Correctness is evaluated server-side and must not be trusted
    -- from student-submitted data.
    is_correct  BOOLEAN NOT NULL DEFAULT FALSE,

    -- Position of the option within the question.
    "order"     INTEGER NOT NULL,

    -- Prevents two options belonging to the same question from
    -- occupying the same position.
    UNIQUE (question_id, "order")
);


-- Stores an individual student's attempt at completing a quiz.
--
-- An attempt represents the lifecycle and aggregate result of one
-- student interaction with a quiz.
CREATE TABLE quiz_attempts
(
    id         UUID PRIMARY KEY,

    quiz_id    UUID        NOT NULL
        REFERENCES quizzes (id) ON DELETE CASCADE,

    -- The student identifier is owned by the student/account service
    -- and is therefore intentionally not a local foreign key.
    student_id UUID        NOT NULL,

    -- Tracks the lifecycle state of the attempt.
    status     TEXT        NOT NULL
        CHECK (status IN ('inProgress', 'completed', 'abandoned')),

    started_at TIMESTAMPTZ NOT NULL,
    ended_at   TIMESTAMPTZ,

    -- Aggregate result calculated from the student's responses.
    score      INTEGER,
    percentage DOUBLE PRECISION,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- Stores the response submitted by a student for an individual
-- question within a quiz attempt.
--
-- Unanswered questions do not create response records. This allows
-- the absence of a response to represent that the student skipped
-- the question.
CREATE TABLE quiz_question_responses
(
    id                 UUID PRIMARY KEY,

    attempt_id         UUID        NOT NULL
        REFERENCES quiz_attempts (id)
            ON DELETE CASCADE,

    question_id        UUID        NOT NULL
        REFERENCES quiz_questions (id)
            ON DELETE CASCADE,

    selected_option_id UUID        NOT NULL
        REFERENCES quiz_question_options (id),

    -- Determined by the server when the response is evaluated.
    is_correct         BOOLEAN     NOT NULL,

    answered_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- A question can only be answered once within a given attempt.
    UNIQUE (attempt_id, question_id)
);


-- Indexes supporting the primary lookup paths for quiz content
-- and student assessment history.

CREATE INDEX idx_quizzes_lesson_id
    ON quizzes(lesson_id);

CREATE INDEX idx_quiz_questions_quiz_id
    ON quiz_questions(quiz_id);

CREATE INDEX idx_quiz_question_options_question_id
    ON quiz_question_options(question_id);

CREATE INDEX idx_quiz_attempts_quiz_id
    ON quiz_attempts(quiz_id);

CREATE INDEX idx_quiz_attempts_student_id
    ON quiz_attempts(student_id);

CREATE INDEX idx_quiz_question_responses_attempt_id
    ON quiz_question_responses(attempt_id);