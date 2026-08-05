use crate::core::lessons::dto::CreateLessonRequest::CreateLessonRequest;
use crate::core::lessons::models::Lesson::Lesson;
use crate::core::lessons::repository::LessonRepository::LessonRepository;
use sqlx::Error;
use uuid::Uuid;

pub struct PostgresLessonRepository {
    pool: sqlx::PgPool,
}

impl PostgresLessonRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl LessonRepository for PostgresLessonRepository {
    async fn get_lesson_by_id(&self, id: Uuid) -> sqlx::Result<Option<Lesson>, Error> {
        sqlx::query_as("SELECT * FROM lessons WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn create_lesson(&self, lesson: &CreateLessonRequest) -> sqlx::Result<Lesson, Error> {
        let id = Uuid::now_v7();
        sqlx::query_as(
            "INSERT INTO lessons(
                        id,
                        video_url,
                        thumbnail_url,
                        transcript_url,
                        duration_seconds,
                        resolution,
                        language,
                        captions_url
                    ) VALUES (
                        $1, $2, $3, $4, $5, $6, $7, $8
                    ) RETURNING *",
        )
        .bind(id)
        .bind(&lesson.video_url)
        .bind(&lesson.thumbnail_url)
        .bind(&lesson.transcript_url)
        .bind(&lesson.duration_seconds)
        .bind(&lesson.resolution)
        .bind(&lesson.language)
        .bind(&lesson.captions_url)
        .fetch_one(&self.pool)
        .await
    }
}
