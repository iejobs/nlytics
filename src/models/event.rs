use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Deserialize, sqlx::FromRow)]
pub struct Event {
    pub id: Uuid,
    pub r#type: String,
    pub data: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventResponse {
    pub id: Uuid,
    pub r#type: String,
    pub data: Value,
}

impl From<Event> for EventResponse {
    fn from(event: Event) -> Self {
        Self {
            id: event.id,
            r#type: event.r#type,
            data: event.data,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEvent {
    pub r#type: String,
    pub data: Value,
}