use crate::core::materials::dto::CreateMaterialRequestDTO::CreateMaterialRequest;
use crate::core::materials::models::Material::Material;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait MaterialRepository {
    async fn get_materials_by_topic_id(
        &self,
        topic_id: &Uuid,
    ) -> sqlx::Result<Vec<Material>, sqlx::Error>;

    async fn create_material(
        &self,
        material: &CreateMaterialRequest,
    ) -> sqlx::Result<Material, sqlx::Error>;
}
