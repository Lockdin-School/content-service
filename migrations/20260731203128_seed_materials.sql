-- Add migration script here
INSERT INTO materials(
            id, code, slug, title, short_description, description, topic_id, material_type
) SELECT
      gen_random_uuid(),
      'MAT11-T01-FN-01',
      'mathematics-grade-11-term-1-function-1',
      'Functions',
      'Introductory functions',
      'An introduction to functions in mathematics.',
      '166f6839-2012-4415-b6a8-88ca8969b890',
      'resource'