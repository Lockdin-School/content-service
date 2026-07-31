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
}
