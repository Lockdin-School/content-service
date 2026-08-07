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
        display_order: i32,
    ) -> sqlx::Result<Material, Error> {
        sqlx::query_as(
            "
                    INSERT INTO materials(material_id, code, slug, title, short_description, description, topic_id, material_type, display_order)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *
                   ")
            .bind(material_id)
            .bind(&material.code)
            .bind(&material.slug)
            .bind(&material.title)
            .bind(&material.short_description)
            .bind(&material.description)
            .bind(material.topic_id)
            .bind(material.material_type)
            .bind(display_order)
            .fetch_one(&self.pool)
            .await
    }

    async fn get_material_by_id(&self, id: &Uuid) -> sqlx::Result<Option<Material>, Error> {
        sqlx::query_as("SELECT * FROM materials WHERE material_id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn next_display_order(&self, topic_id: &Uuid) -> sqlx::Result<i32, Error> {
        let next: i32 = sqlx::query_scalar(
            r#"
            SELECT COALESCE(MAX(display_order), 0) + 1
            FROM materials
            WHERE topic_id = $1
            "#,
        )
        .bind(topic_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(next)
    }
}
