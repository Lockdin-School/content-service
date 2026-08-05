-- Add migration script here
CREATE TABLE lessons (
    -- Identity
                         material_id UUID PRIMARY KEY,

    -- Video
                         video_url TEXT NOT NULL,
                         thumbnail_url TEXT,
                         transcript_url TEXT,

    -- Playback
                         duration_seconds INT NOT NULL,
                         resolution VARCHAR(20),
                         language VARCHAR(10) DEFAULT 'en',

    -- Optional captions
                         captions_url TEXT,

    -- Analytics
                         watch_count BIGINT NOT NULL DEFAULT 0,

    -- Audit
                         created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                         updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Relationships
                         CONSTRAINT fk_lessons_material
                             FOREIGN KEY (material_id)
                                 REFERENCES materials(id)
                                 ON DELETE CASCADE
);

CREATE INDEX idx_lessons_watch_count
    ON lessons (watch_count);

CREATE INDEX idx_lessons_created_at
    ON lessons (created_at);