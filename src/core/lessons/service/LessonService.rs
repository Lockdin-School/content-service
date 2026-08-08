use crate::core::lessons::dto::CreateLessonRequest::{
    CreateLessonRequest, IncomingCreateLessonRequest,
};
use crate::core::lessons::lessons_events::LessonCreatedPayload;
use crate::core::lessons::models::Lesson::Lesson;
use crate::core::lessons::repository::LessonRepository::LessonRepository;
use crate::infrastructure::InternalEventBus::{Event, EventBus};
use crate::utils::code::generate_code;
use crate::utils::slug::generate_slug;
use actix_web::web::Data;
use std::io::Error;
use std::sync::Arc;

pub struct LessonService {
    repo: Arc<dyn LessonRepository + Send + Sync>,
    event_bus: Data<EventBus>,
}

impl LessonService {
    pub fn new(repo: Arc<dyn LessonRepository + Send + Sync>, event_bus: Data<EventBus>) -> Self {
        Self { repo, event_bus }
    }

    pub async fn get_lesson_by_id(&self, id: uuid::Uuid) -> Result<Lesson, Error> {
        match self.repo.get_lesson_by_id(id).await {
            Ok(lesson) => match lesson {
                Some(lesson) => {
                    log::info!("lesson.get | service | get_lesson_by_id | success | \"Lesson found\" |");
                    Ok(lesson)
                },
                None => {
                    log::error!("lesson.get | service | get_lesson_by_id | failure | \"Lesson not found\" |");
                    Err(Error::new(std::io::ErrorKind::NotFound, "Lesson not found"))
                }
            },
            Err(e) => {
                log::error!("lesson.get | service | get_lesson_by_id | failure | \"{:?}\" |", e);
                Err(Error::other(e.to_string()))
            }
        }
    }

    pub async fn create_lesson(
        &self,
        incoming_lesson: &IncomingCreateLessonRequest,
    ) -> Result<Lesson, Error> {
        log::info!("lessons.create | service | create_lesson | started | \"Creating lesson\" |");

        let lesson = incoming_lesson.clone();

        let lesson_request = CreateLessonRequest {
            video_url: lesson.video_url,
            thumbnail_url: lesson.thumbnail_url,
            transcript_url: lesson.transcript_url,
            duration_seconds: lesson.duration_seconds,
            resolution: lesson.resolution,
            language: lesson.language,
            captions_url: lesson.captions_url,
        };

        match self.repo.create_lesson(&lesson_request).await {
            Ok(lesson) => {
                log::info!(
                    "lessons.create | service | create_lesson | success | \"Lesson created\" |"
                );

                let lesson_created_payload = LessonCreatedPayload {
                    material_id: lesson.id,
                    code: generate_code("L"),
                    slug: generate_slug("L", &incoming_lesson.title),
                    title: incoming_lesson.title.clone(),
                    short_description: incoming_lesson.short_description.clone(),
                    description: incoming_lesson.description.clone(),
                    topic_id: incoming_lesson.topic_id,
                    estimated_duration_seconds: Some(incoming_lesson.duration_seconds),
                    is_featured: incoming_lesson.is_featured,
                    is_free: incoming_lesson.is_free,
                };

                if let Err(e) = self
                    .event_bus
                    .send(Event::LessonCreated(lesson_created_payload))
                {
                    log::error!(
                        "lessons.create | internal_event_bus | create_lesson | failed | \"Failed to publish lesson created event\" | {:?}",
                        e
                    );
                };

                Ok(lesson)
            }
            Err(e) => Err(Error::other(e.to_string())),
        }
    }
}
