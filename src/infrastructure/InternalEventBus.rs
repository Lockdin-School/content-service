use crate::core::concepts::concepts_events::ConceptCreatedPayload;
use crate::core::lessons::lessons_events::LessonCreatedPayload;
use crate::core::quizzes::quizzes_events::QuizCreatedPayload;
use tokio::sync::broadcast;

pub type EventBus = broadcast::Sender<Event>;

#[derive(Debug, Clone)]
pub enum Event {
    LessonCreated(LessonCreatedPayload),
    QuizCreated(QuizCreatedPayload),
    ConceptCreated(ConceptCreatedPayload),
}

pub fn init_event_bus() -> broadcast::Sender<Event> {
    let (tx, _rx) = broadcast::channel(16);
    tx
}
