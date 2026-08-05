use tokio::sync::broadcast;
use crate::core::lessons::lessons_events::LessonCreatedPayload;

pub type EventBus = broadcast::Sender<Event>;

#[derive(Debug, Clone)]
pub enum Event {
    LessonCreated(LessonCreatedPayload),
}

pub fn init_event_bus() -> broadcast::Sender<Event> {
    let (tx, _rx) = broadcast::channel(16);
    tx
}