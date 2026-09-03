use crate::core::concepts::models::Concept::{Concept, ConceptNew, ConceptUpdate};
use async_trait::async_trait;
use sqlx::Result;
use uuid::Uuid;

#[async_trait]
pub trait ConceptRepository: Send + Sync {
    async fn create(&self, concept: ConceptNew) -> Result<Concept, sqlx::Error>;

    async fn create_many(&self, concepts: Vec<ConceptNew>) -> Result<Vec<Concept>, sqlx::Error>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Concept>, sqlx::Error>;

    async fn find_by_topic_id(&self, topic_id: Uuid) -> Result<Vec<Concept>, sqlx::Error>;

    async fn find_all(&self) -> Result<Vec<Concept>, sqlx::Error>;

    async fn update(
        &self,
        id: Uuid,
        concept: ConceptUpdate,
    ) -> Result<Option<Concept>, sqlx::Error>;

    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
}
