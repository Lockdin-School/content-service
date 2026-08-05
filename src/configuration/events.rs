use std::sync::Arc;
use actix_web::web::Data;
use tokio::sync::broadcast::Sender;
use crate::configuration::state::AppState;
use crate::infrastructure::InternalEventBus::Event;

pub async fn event_handlers_init(tx: Sender<Event>, state: Data<AppState>) {
    // --- SPAWN MATERIAL SUBSCRIBER ---
    let materials_service = Arc::new(state.material_service.clone());
    let rx_materials = tx.subscribe();
    let materials_clone = Arc::clone(&materials_service);
    tokio::spawn(async move {
        materials_clone.materials_events_handler(rx_materials).await;
    });
}