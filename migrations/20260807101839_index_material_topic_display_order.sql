-- Add migration script here
CREATE INDEX idx_materials_topic_display_order
    ON materials (topic_id, display_order);