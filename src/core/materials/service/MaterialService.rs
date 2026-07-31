use std::io::Error;
use std::sync::Arc;
use uuid::Uuid;
use crate::core::materials::models::Material::Material;
use crate::core::materials::repository::MaterialRepository::MaterialRepository;


pub struct MaterialService {
    repo: Arc<dyn MaterialRepository + Send + Sync>
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
                    error.to_string()
                );
                Err(Error::other(error.to_string()))
            }
        }
    }
}