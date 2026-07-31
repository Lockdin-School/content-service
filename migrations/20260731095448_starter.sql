-- Add migration script here
CREATE TYPE material_type AS ENUM (
    'lesson',
    'resource',
    'exercise',
    'quiz',
    'assignment'
    );

CREATE TYPE material_status AS ENUM (
    'draft',
    'published',
    'archived'
    );

CREATE TABLE materials (
    --------------------------------------------------------------------------
    -- Identity
    --------------------------------------------------------------------------
                           id UUID PRIMARY KEY,

                           code VARCHAR(50) NOT NULL UNIQUE,
                           slug VARCHAR(255) NOT NULL UNIQUE,

                           title VARCHAR(255) NOT NULL,
                           short_description VARCHAR(160),
                           description TEXT,

    --------------------------------------------------------------------------
    -- Relationships
    --------------------------------------------------------------------------
                           topic_id UUID NOT NULL,

    --------------------------------------------------------------------------
    -- Classification
    --------------------------------------------------------------------------
                           material_type material_type NOT NULL,

    --------------------------------------------------------------------------
    -- Presentation
    --------------------------------------------------------------------------

                           display_order INTEGER NOT NULL DEFAULT 0,

                           estimated_duration_seconds INTEGER,

    --------------------------------------------------------------------------
    -- Cached Statistics
    --------------------------------------------------------------------------
                           view_count BIGINT NOT NULL DEFAULT 0,

    --------------------------------------------------------------------------
    -- Status
    --------------------------------------------------------------------------
                           status material_status NOT NULL DEFAULT 'draft',

                           is_featured BOOLEAN NOT NULL DEFAULT FALSE,
                           is_free BOOLEAN NOT NULL DEFAULT FALSE,

    --------------------------------------------------------------------------
    -- Versioning
    --------------------------------------------------------------------------
                           version INTEGER NOT NULL DEFAULT 1,

    --------------------------------------------------------------------------
    -- Audit
    --------------------------------------------------------------------------
                           created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                           updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

                           published_at TIMESTAMPTZ,
                           archived_at TIMESTAMPTZ,
                           deleted_at TIMESTAMPTZ,

    --------------------------------------------------------------------------
    -- Constraints
    --------------------------------------------------------------------------
                           CONSTRAINT chk_material_display_order
                               CHECK (display_order >= 0),

                           CONSTRAINT chk_material_duration
                               CHECK (
                                   estimated_duration_seconds IS NULL
                                       OR estimated_duration_seconds >= 0
                                   ),

                           CONSTRAINT chk_material_view_count
                               CHECK (view_count >= 0),

                           CONSTRAINT chk_material_version
                               CHECK (version >= 1)
);

------------------------------------------------------------------------------
-- INDEXES
------------------------------------------------------------------------------

CREATE INDEX idx_materials_topic
    ON materials(topic_id);

CREATE INDEX idx_materials_topic_order
    ON materials(topic_id, display_order);

CREATE INDEX idx_materials_type
    ON materials(material_type);

CREATE INDEX idx_materials_status
    ON materials(status);

CREATE INDEX idx_materials_topic_type
    ON materials(topic_id, material_type);

CREATE INDEX idx_materials_active
    ON materials(deleted_at)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_materials_featured
    ON materials(is_featured)
    WHERE is_featured = TRUE;

CREATE INDEX idx_materials_free
    ON materials(is_free)
    WHERE is_free = TRUE;

CREATE INDEX idx_materials_updated_at
    ON materials(updated_at DESC);