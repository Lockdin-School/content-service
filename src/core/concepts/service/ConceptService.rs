use crate::core::concepts::concepts_events::ConceptCreatedPayload;
use crate::core::concepts::models::Concept::{Concept, ConceptNew, ConceptUpdate};
use crate::core::concepts::repository::ConceptRepository::ConceptRepository;
use crate::infrastructure::InternalEventBus::{Event, EventBus};
use crate::utils::code::generate_code;
use crate::utils::slug::generate_slug;
use actix_web::web::Data;
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use uuid::Uuid;

pub struct ConceptService {
    repo: Arc<dyn ConceptRepository + Send + Sync>,
    event_bus: Data<EventBus>,
}

impl ConceptService {
    pub fn new(repo: Arc<dyn ConceptRepository + Send + Sync>, event_bus: Data<EventBus>) -> Self {
        Self { repo, event_bus }
    }

    pub async fn create_concept(&self, concept: ConceptNew) -> Result<Concept, Error> {
        log::info!(
            "concept.create.start | service | create_concept | started | \"Creating concept.\" |"
        );

        match self.repo.create(concept).await {
            Ok(concept) => {
                log::info!(
                    "concept.create.success | service | create_concept | success | \"Created concept successfully.\" | concept_id={}",
                    concept.id
                );

                let concept_created_payload = ConceptCreatedPayload {
                    material_id: concept.id,
                    code: generate_code("C"),
                    slug: generate_slug("C", &concept.title),
                    title: concept.title.clone(),
                    short_description: concept.summary.clone(),
                    description: concept.summary.clone(),
                    topic_id: concept.topic_id,
                    estimated_duration_seconds: None,
                    is_featured: false,
                    is_free: false,
                };

                if let Err(error) = self
                    .event_bus
                    .send(Event::ConceptCreated(concept_created_payload))
                {
                    log::error!(
                        "concept.create | internal_event_bus | create_concept | failed | \"Failed to publish concept created event\" | {:?}",
                        error
                    );
                }

                Ok(concept)
            }
            Err(error) => {
                log::error!(
                    "concept.create.failed | service | create_concept | failed | \"Failed creating concept.\" | error=\"{}\"",
                    error
                );

                Err(Error::other(error.to_string()))
            }
        }
    }

    pub async fn create_concepts_bulk(
        &self,
        concepts: Vec<ConceptNew>,
    ) -> Result<Vec<Concept>, Error> {
        log::info!(
            "concepts.bulk_create.start | service | create_concepts_bulk | started | \"Creating concepts in bulk.\" | count={}",
            concepts.len()
        );

        if concepts.is_empty() {
            log::warn!(
                "concepts.bulk_create.failed | service | create_concepts_bulk | failed | \"Empty batch received.\" |"
            );
            return Err(Error::new(ErrorKind::InvalidInput, "No concepts provided."));
        }

        match self.repo.create_many(concepts).await {
            Ok(created) => {
                log::info!(
                    "concepts.bulk_create.success | service | create_concepts_bulk | success | \"Created concepts successfully.\" | count={}",
                    created.len()
                );

                // Publish events for each created concept
                for concept in &created {
                    let payload = ConceptCreatedPayload {
                        material_id: concept.id,
                        code: generate_code("C"),
                        slug: generate_slug("C", &concept.title),
                        title: concept.title.clone(),
                        short_description: concept.summary.clone(),
                        description: concept.summary.clone(),
                        topic_id: concept.topic_id,
                        estimated_duration_seconds: None,
                        is_featured: false,
                        is_free: false,
                    };

                    if let Err(error) = self.event_bus.send(Event::ConceptCreated(payload)) {
                        log::error!(
                            "concepts.bulk_create | internal_event_bus | create_concepts_bulk | failed | \"Failed to publish concept created event\" | concept_id={} | {:?}",
                            concept.id,
                            error
                        );
                    }
                }

                Ok(created)
            }
            Err(error) => {
                log::error!(
                    "concepts.bulk_create.failed | service | create_concepts_bulk | failed | \"Failed creating concepts in bulk.\" | error=\"{}\"",
                    error
                );

                Err(Error::other(error.to_string()))
            }
        }
    }

    pub async fn get_concept_by_id(&self, id: &Uuid) -> Result<Concept, Error> {
        log::info!(
            "concept.get.start | service | get_concept_by_id | started | \"Getting concept.\" | concept_id={}",
            id
        );

        match self.repo.find_by_id(*id).await {
            Ok(Some(concept)) => {
                log::info!(
                    "concept.get.success | service | get_concept_by_id | success | \"Returned concept successfully.\" | concept_id={}",
                    concept.id
                );

                Ok(concept)
            }
            Ok(None) => {
                log::info!(
                    "concept.get.failed | service | get_concept_by_id | failed | \"Concept not found.\" | concept_id={}",
                    id
                );

                Err(Error::new(ErrorKind::NotFound, "Concept not found."))
            }
            Err(error) => {
                log::error!(
                    "concept.get.failed | service | get_concept_by_id | failed | \"Failed retrieving concept.\" | concept_id={} | error=\"{}\"",
                    id,
                    error
                );

                Err(Error::other(error.to_string()))
            }
        }
    }

    pub async fn get_concepts_by_topic_id(&self, topic_id: &Uuid) -> Result<Vec<Concept>, Error> {
        log::info!(
            "concepts.get.start | service | get_concepts_by_topic_id | started | \"Getting concepts by topic.\" | topic_id={}",
            topic_id
        );

        match self.repo.find_by_topic_id(*topic_id).await {
            Ok(concepts) => {
                log::info!(
                    "concepts.get.success | service | get_concepts_by_topic_id | success | \"Returned concepts successfully.\" | topic_id={} | count={}",
                    topic_id,
                    concepts.len()
                );

                Ok(concepts)
            }
            Err(error) => {
                log::error!(
                    "concepts.get.failed | service | get_concepts_by_topic_id | failed | \"Failed retrieving concepts by topic.\" | topic_id={} | error=\"{}\"",
                    topic_id,
                    error
                );

                Err(Error::other(error.to_string()))
            }
        }
    }

    pub async fn get_all_concepts(&self) -> Result<Vec<Concept>, Error> {
        log::info!(
            "concepts.get.start | service | get_all_concepts | started | \"Getting all concepts.\" |"
        );

        match self.repo.find_all().await {
            Ok(concepts) => {
                log::info!(
                    "concepts.get.success | service | get_all_concepts | success | \"Returned concepts successfully.\" | count={}",
                    concepts.len()
                );

                Ok(concepts)
            }
            Err(error) => {
                log::error!(
                    "concepts.get.failed | service | get_all_concepts | failed | \"Failed retrieving concepts.\" | error=\"{}\"",
                    error
                );

                Err(Error::other(error.to_string()))
            }
        }
    }

    pub async fn update_concept(
        &self,
        id: &Uuid,
        concept: ConceptUpdate,
    ) -> Result<Concept, Error> {
        log::info!(
            "concept.update.start | service | update_concept | started | \"Updating concept.\" | concept_id={}",
            id
        );

        match self.repo.update(*id, concept).await {
            Ok(Some(concept)) => {
                log::info!(
                    "concept.update.success | service | update_concept | success | \"Updated concept successfully.\" | concept_id={}",
                    concept.id
                );

                Ok(concept)
            }
            Ok(None) => {
                log::info!(
                    "concept.update.failed | service | update_concept | failed | \"Concept not found.\" | concept_id={}",
                    id
                );

                Err(Error::new(ErrorKind::NotFound, "Concept not found."))
            }
            Err(error) => {
                log::error!(
                    "concept.update.failed | service | update_concept | failed | \"Failed updating concept.\" | concept_id={} | error=\"{}\"",
                    id,
                    error
                );

                Err(Error::other(error.to_string()))
            }
        }
    }

    pub async fn delete_concept(&self, id: &Uuid) -> Result<(), Error> {
        log::info!(
            "concept.delete.start | service | delete_concept | started | \"Deleting concept.\" | concept_id={}",
            id
        );

        match self.repo.delete(*id).await {
            Ok(true) => {
                log::info!(
                    "concept.delete.success | service | delete_concept | success | \"Deleted concept successfully.\" | concept_id={}",
                    id
                );

                Ok(())
            }
            Ok(false) => {
                log::info!(
                    "concept.delete.failed | service | delete_concept | failed | \"Concept not found.\" | concept_id={}",
                    id
                );

                Err(Error::new(ErrorKind::NotFound, "Concept not found."))
            }
            Err(error) => {
                log::error!(
                    "concept.delete.failed | service | delete_concept | failed | \"Failed deleting concept.\" | concept_id={} | error=\"{}\"",
                    id,
                    error
                );

                Err(Error::other(error.to_string()))
            }
        }
    }
}
