mod create_event;
mod get_event;
mod get_events;

pub use create_event::*;
pub use get_event::*;
pub use get_events::*;

#[derive(serde::Deserialize)]
pub struct EventPath {
    pub event_id: String,
}
