use crate::core::materials::dto::CreateMaterialRequestDTO::CreateMaterialRequest;
use crate::core::materials::models::Material::Material;
use crate::core::materials::models::MaterialType::MaterialType;
use crate::core::materials::repository::MaterialRepository::MaterialRepository;
use crate::infrastructure::InternalEventBus::Event;
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use uuid::Uuid;

pub struct MaterialService {
    repo: Arc<dyn MaterialRepository + Send + Sync>,
}

impl MaterialService {
    pub fn new(repo: Arc<dyn MaterialRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    // ---------------- EVENT HANDLER --------------------- //
    pub async fn materials_events_handler(
        &self,
        mut receiver: tokio::sync::broadcast::Receiver<Event>,
    ) {
        log::info!(
            "materials.events.listen | service | materials_events_handler | started | \"Listening for events\" |"
        );

        while let Ok(event) = receiver.recv().await {
            match event.clone() {
                Event::LessonCreated(payload) => {
                    log::info!(
                        "materials.events.listen | service | materials_events_handler | success | \"Received event: LessonCreated.\" |"
                    );

                    let material_request = CreateMaterialRequest {
                        code: payload.code,
                        slug: payload.slug,
                        title: payload.title,
                        short_description: payload.short_description,
                        description: payload.description,
                        topic_id: payload.topic_id,
                        material_type: MaterialType::Lesson,
                        estimated_duration_seconds: None,
                        is_featured: false,
                        is_free: false,
                    };

                    // Trigger material creation
                    match self
                        .create_material(&payload.material_id, material_request)
                        .await
                    {
                        Ok(_) => {
                            log::info!(
                                "materials.events.listen | service | materials_events_handler | success | \"Material created\" |"
                            );
                        }
                        Err(e) => {
                            log::error!(
                                "materials.events.listen | service | materials_events_handler | failed | \"Failed to create material\" | error=\"{}\"",
                                e
                            );
                        }
                    }
                }
            }
        }
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
        material_id: &Uuid,
        material: CreateMaterialRequest,
    ) -> Result<Material, Error> {
        log::info!(
            "material.create.start | service | create_material | started | \"Creating material.\" |"
        );
        let display_order = self.next_display_order(&material.topic_id).await?;
        match self.repo.create_material(material_id, &material, display_order).await {
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

    pub async fn get_material_by_id(&self, id: &Uuid) -> Result<Material, Error> {
        log::info!(
            "material.get.start | service | get_material_by_id | started | \"Getting material.\" |"
        );
        match self.repo.get_material_by_id(&id).await {
            Ok(opt_material) => match opt_material {
                Some(material) => {
                    log::info!(
                        "material.get.success | service | get_material_by_id | success | \"Returned material successfully.\" |",
                    );
                    Ok(material)
                }
                None => {
                    log::info!(
                        "material.get.failed | service | get_material_by_id | failed | \"Material not found.\" |",
                    );
                    Err(Error::new(ErrorKind::NotFound, "Material not found."))
                }
            },
            Err(error) => {
                log::error!(
                    "material.get.failed | service | get_material_by_id | failed | \"Failed retrieving material.\" | error=\"{}\"",
                    error
                );
                Err(Error::other(error.to_string()))
            }
        }
    }

    pub async fn next_display_order(&self, topic_id: &Uuid) -> Result<i32, Error> {
        log::info!(
            "materials.get.started | service | next_display_order | started | \"Getting next display order\" |"
        );
        match self.repo.next_display_order(&topic_id).await {
            Ok(display_order) => {
                log::info!(
                    "materials.get.started | service | next_display_order | success | \"Returned next display order successfully\" | display_order={}",
                    display_order
                );
                Ok(display_order)
            }
            Err(e) => {
                log::error!(
                    "materials.get.started | service | next_display_order | failed | \"Failed to get next display order\" | error=\"{}\"",
                    e
                );
                Err(Error::other(e.to_string()))
            }
        }
    }
}
