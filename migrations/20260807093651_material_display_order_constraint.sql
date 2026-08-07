-- Add migration script here
ALTER TABLE materials
    ADD CONSTRAINT uq_materials_topic_display_order
        UNIQUE (topic_id, display_order);