-- Add migration script here
CREATE TABLE concepts (
                          id              UUID PRIMARY KEY,
                          topic_id        UUID NOT NULL,

                          title           TEXT NOT NULL,
                          display_order   INTEGER NOT NULL DEFAULT 0,

                          definitions     JSONB NOT NULL DEFAULT '[]',
                          explanation     JSONB NOT NULL DEFAULT '[]',
                          analogy         JSONB,

                          examples        JSONB NOT NULL DEFAULT '[]',
                          misconceptions  JSONB NOT NULL DEFAULT '[]',

                          summary         TEXT,

                          created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                          updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_concepts_topic_order
    ON concepts (topic_id, display_order);