use async_trait::async_trait;
use crate::models::event::{Event, CreateEvent};
use sqlx::Error;

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Event>, Error>;
    async fn create(&self, event: CreateEvent) -> Result<Event, Error>;
}