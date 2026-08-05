-- Add migration script here
BEGIN;

-- ============================================================================
-- LESSONS
-- Remove material_id and introduce its own primary key
-- ============================================================================

ALTER TABLE lessons
    DROP CONSTRAINT lessons_pkey;

ALTER TABLE lessons
    DROP COLUMN material_id;

ALTER TABLE lessons
    ADD COLUMN id UUID PRIMARY KEY DEFAULT gen_random_uuid();

-- ============================================================================
-- MATERIALS
-- Rename id -> material_id
-- ============================================================================

ALTER TABLE materials
    RENAME COLUMN id TO material_id;

COMMIT;