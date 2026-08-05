use crate::core::materials::dto::CreateMaterialRequestDTO::CreateMaterialRequest;
use crate::core::materials::models::Material::Material;
use crate::core::materials::repository::MaterialRepository::MaterialRepository;
use sqlx::Error;
use uuid::Uuid;

pub struct PostgresMaterialRepository {
    pool: sqlx::PgPool,
}

impl PostgresMaterialRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl MaterialRepository for PostgresMaterialRepository {
    async fn get_materials_by_topic_id(
        &self,
        topic_id: &Uuid,
    ) -> sqlx::Result<Vec<Material>, Error> {
        sqlx::query_as("SELECT * FROM materials WHERE topic_id = $1")
            .bind(topic_id)
            .fetch_all(&self.pool)
            .await
    }

    async fn create_material(
        &self,
        material_id: &Uuid,
        material: &CreateMaterialRequest,
    ) -> sqlx::Result<Material, Error> {
        sqlx::query_as(
            "
                    INSERT INTO materials(material_id, code, slug, title, short_description, description, topic_id, material_type)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *
                   ")
            .bind(material_id)
            .bind(&material.code)
            .bind(&material.slug)
            .bind(&material.title)
            .bind(&material.short_description)
            .bind(& material.description)
            .bind(material.topic_id)
            .bind(material.material_type)
            .fetch_one(&self.pool)
            .await
    }

    async fn get_material_by_id(
        &self,
        id: &Uuid
    ) -> sqlx::Result<Option<Material>, Error> {
        sqlx::query_as("SELECT * FROM materials WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
}
