use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::core::concepts::models::Concept::{Concept, ConceptNew, ConceptUpdate};
use crate::core::concepts::models::Definition::Definition;
use crate::core::concepts::models::Example::Example;
use crate::core::concepts::models::ExplanationSection::ExplanationSection;
use crate::core::concepts::models::Misconception::Misconception;
use crate::core::concepts::repository::ConceptRepository::ConceptRepository;

pub struct PostgresConceptRepository {
    pool: PgPool,
}

impl PostgresConceptRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_row(row: PgRow) -> Result<Concept, sqlx::Error> {
        let definitions: serde_json::Value = row.try_get("definitions")?;
        let explanation: serde_json::Value = row.try_get("explanation")?;
        let analogy: Option<serde_json::Value> = row.try_get("analogy")?;
        let examples: serde_json::Value = row.try_get("examples")?;
        let misconceptions: serde_json::Value = row.try_get("misconceptions")?;

        Ok(Concept {
            id: row.try_get("id")?,
            topic_id: row.try_get("topic_id")?,
            title: row.try_get("title")?,
            display_order: row.try_get("display_order")?,

            definitions: serde_json::from_value::<Vec<Definition>>(definitions)
                .map_err(|err| sqlx::Error::Decode(Box::new(err)))?,

            explanation: serde_json::from_value::<Vec<ExplanationSection>>(explanation)
                .map_err(|err| sqlx::Error::Decode(Box::new(err)))?,

            analogy: match analogy {
                Some(value) => Some(
                    serde_json::from_value::<Vec<ExplanationSection>>(value)
                        .map_err(|err| sqlx::Error::Decode(Box::new(err)))?,
                ),
                None => None,
            },

            examples: serde_json::from_value::<Vec<Example>>(examples)
                .map_err(|err| sqlx::Error::Decode(Box::new(err)))?,

            misconceptions: serde_json::from_value::<Vec<Misconception>>(misconceptions)
                .map_err(|err| sqlx::Error::Decode(Box::new(err)))?,

            summary: row.try_get("summary")?,
            created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
            updated_at: row.try_get::<DateTime<Utc>, _>("updated_at")?,
        })
    }
}

#[async_trait]
impl ConceptRepository for PostgresConceptRepository {
    async fn create(&self, concept: ConceptNew) -> Result<Concept, sqlx::Error> {
        let id = Uuid::new_v4();

        let row = sqlx::query(
            r#"
            INSERT INTO concepts (
                id,
                topic_id,
                title,
                display_order,
                definitions,
                explanation,
                analogy,
                examples,
                misconceptions,
                summary
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5::jsonb,
                $6::jsonb,
                $7::jsonb,
                $8::jsonb,
                $9::jsonb,
                $10
            )
            RETURNING
                id,
                topic_id,
                title,
                display_order,
                definitions,
                explanation,
                analogy,
                examples,
                misconceptions,
                summary,
                created_at,
                updated_at
            "#,
        )
        .bind(id)
        .bind(concept.topic_id)
        .bind(concept.title)
        .bind(concept.display_order)
        .bind(
            serde_json::to_value(concept.definitions)
                .map_err(|err| sqlx::Error::Encode(Box::new(err)))?,
        )
        .bind(
            serde_json::to_value(concept.explanation)
                .map_err(|err| sqlx::Error::Encode(Box::new(err)))?,
        )
        .bind(
            concept
                .analogy
                .map(serde_json::to_value)
                .transpose()
                .map_err(|err| sqlx::Error::Encode(Box::new(err)))?,
        )
        .bind(
            serde_json::to_value(concept.examples)
                .map_err(|err| sqlx::Error::Encode(Box::new(err)))?,
        )
        .bind(
            serde_json::to_value(concept.misconceptions)
                .map_err(|err| sqlx::Error::Encode(Box::new(err)))?,
        )
        .bind(concept.summary)
        .fetch_one(&self.pool)
        .await?;

        Self::map_row(row)
    }

    async fn create_many(&self, concepts: Vec<ConceptNew>) -> Result<Vec<Concept>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let mut created = Vec::with_capacity(concepts.len());

        for concept in concepts {
            let id = Uuid::new_v4();

            let row = sqlx::query(
                r#"
            INSERT INTO concepts (
                id,
                topic_id,
                title,
                display_order,
                definitions,
                explanation,
                analogy,
                examples,
                misconceptions,
                summary
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5::jsonb,
                $6::jsonb,
                $7::jsonb,
                $8::jsonb,
                $9::jsonb,
                $10
            )
            RETURNING
                id,
                topic_id,
                title,
                display_order,
                definitions,
                explanation,
                analogy,
                examples,
                misconceptions,
                summary,
                created_at,
                updated_at
            "#,
            )
            .bind(id)
            .bind(concept.topic_id)
            .bind(concept.title)
            .bind(concept.display_order)
            .bind(
                serde_json::to_value(concept.definitions)
                    .map_err(|err| sqlx::Error::Encode(Box::new(err)))?,
            )
            .bind(
                serde_json::to_value(concept.explanation)
                    .map_err(|err| sqlx::Error::Encode(Box::new(err)))?,
            )
            .bind(
                concept
                    .analogy
                    .map(serde_json::to_value)
                    .transpose()
                    .map_err(|err| sqlx::Error::Encode(Box::new(err)))?,
            )
            .bind(
                serde_json::to_value(concept.examples)
                    .map_err(|err| sqlx::Error::Encode(Box::new(err)))?,
            )
            .bind(
                serde_json::to_value(concept.misconceptions)
                    .map_err(|err| sqlx::Error::Encode(Box::new(err)))?,
            )
            .bind(concept.summary)
            .fetch_one(&mut *tx)
            .await?;

            created.push(Self::map_row(row)?);
        }

        tx.commit().await?;

        Ok(created)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Concept>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                topic_id,
                title,
                display_order,
                definitions,
                explanation,
                analogy,
                examples,
                misconceptions,
                summary,
                created_at,
                updated_at
            FROM concepts
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        row.map(Self::map_row).transpose()
    }

    async fn find_by_topic_id(&self, topic_id: Uuid) -> Result<Vec<Concept>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                topic_id,
                title,
                display_order,
                definitions,
                explanation,
                analogy,
                examples,
                misconceptions,
                summary,
                created_at,
                updated_at
            FROM concepts
            WHERE topic_id = $1
            ORDER BY display_order , created_at
            "#,
        )
        .bind(topic_id)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(Self::map_row).collect()
    }

    async fn find_all(&self) -> Result<Vec<Concept>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                topic_id,
                title,
                display_order,
                definitions,
                explanation,
                analogy,
                examples,
                misconceptions,
                summary,
                created_at,
                updated_at
            FROM concepts
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(Self::map_row).collect()
    }

    async fn update(
        &self,
        id: Uuid,
        concept: ConceptUpdate,
    ) -> Result<Option<Concept>, sqlx::Error> {
        let existing = self.find_by_id(id).await?;

        let Some(existing) = existing else {
            return Ok(None);
        };

        let topic_id = concept.topic_id.unwrap_or(existing.topic_id);
        let title = concept.title.unwrap_or(existing.title);
        let display_order = concept.display_order.unwrap_or(existing.display_order);

        let definitions = concept.definitions.unwrap_or(existing.definitions);
        let explanation = concept.explanation.unwrap_or(existing.explanation);
        let analogy = concept.analogy.unwrap_or(existing.analogy);
        let examples = concept.examples.unwrap_or(existing.examples);
        let misconceptions = concept.misconceptions.unwrap_or(existing.misconceptions);
        let summary = concept.summary.unwrap_or(existing.summary);

        let row = sqlx::query(
            r#"
            UPDATE concepts
            SET
                topic_id = $2,
                title = $3,
                display_order = $4,
                definitions = $5::jsonb,
                explanation = $6::jsonb,
                analogy = $7::jsonb,
                examples = $8::jsonb,
                misconceptions = $9::jsonb,
                summary = $10,
                updated_at = NOW()
            WHERE id = $1
            RETURNING
                id,
                topic_id,
                title,
                display_order,
                definitions,
                explanation,
                analogy,
                examples,
                misconceptions,
                summary,
                created_at,
                updated_at
            "#,
        )
        .bind(id)
        .bind(topic_id)
        .bind(title)
        .bind(display_order)
        .bind(serde_json::to_value(definitions).map_err(|err| sqlx::Error::Encode(Box::new(err)))?)
        .bind(serde_json::to_value(explanation).map_err(|err| sqlx::Error::Encode(Box::new(err)))?)
        .bind(
            analogy
                .map(serde_json::to_value)
                .transpose()
                .map_err(|err| sqlx::Error::Encode(Box::new(err)))?,
        )
        .bind(serde_json::to_value(examples).map_err(|err| sqlx::Error::Encode(Box::new(err)))?)
        .bind(
            serde_json::to_value(misconceptions)
                .map_err(|err| sqlx::Error::Encode(Box::new(err)))?,
        )
        .bind(summary)
        .fetch_one(&self.pool)
        .await?;

        Ok(Some(Self::map_row(row)?))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM concepts
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
