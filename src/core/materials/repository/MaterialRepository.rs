use uuid::Uuid;
use crate::core::materials::models::Material::Material;

#[async_trait::async_trait]
pub trait MaterialRepository {
    async fn get_materials_by_topic_id(&self, topic_id: &Uuid) -> sqlx::Result<Vec<Material>, sqlx::Error>;
}