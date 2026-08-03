use crate::core::materials::dto::CreateMaterialRequestDTO::CreateMaterialRequest;
use crate::core::materials::models::Material::Material;
use crate::core::materials::repository::MaterialRepository::MaterialRepository;
use std::io::Error;
use std::sync::Arc;
use uuid::Uuid;

pub struct MaterialService {
    repo: Arc<dyn MaterialRepository + Send + Sync>,
}

impl MaterialService {
    pub fn new(repo: Arc<dyn MaterialRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn get_materials_by_topic(&self, topic_id: &Uuid) -> Result<Vec<Material>, Error> {
        log::info!(
            "materials.get.start | service | get_materials_by_topic | started | \"Getting materials\" |"
        );
        match self.repo.get_materials_by_topic_id(topic_id).await {
            Ok(materials) => {
                log::info!(
                    "materials.get.success | service | get_materials_by_topic | success | \"Returned materials successfully\" | count={}",
                    materials.len()
                );
                Ok(materials)
            }
            Err(error) => {
                log::error!(
                    "materials.get.failed | service | get_materials_by_topic | failed | \"Failed to get materials\" | error=\"{}\"",
                    error
                );
                Err(Error::other(error.to_string()))
            }
        }
    }

    pub async fn create_material(
        &self,
        material: CreateMaterialRequest,
    ) -> Result<Material, Error> {
        log::info!(
            "material.create.start | service | create_material | started | \"Creating material.\" |"
        );
        match self.repo.create_material(&material).await {
            Ok(material) => {
                log::info!(
                    "material.create.success | service | create_material | success | \"Created material successfully.\" |",
                );
                Ok(material)
            }
            Err(error) => {
                log::error!(
                    "material.create.failed | service | create_material | failed | \"Failed creating material.\" | error=\"{}\"",
                    error
                );
                Err(Error::other(error.to_string()))
            }
        }
    }
}
