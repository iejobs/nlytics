use crate::models::{Event, CreateEvent};
use sqlx::Error;
use crate::repositories::{EventRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct EventService {
    repo: Arc<dyn EventRepository>,
}

impl EventService {
    pub fn new(repo: Arc<dyn EventRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Event>, Error> {
        self.repo.get_all().await
    }

    pub async fn create(&self, event: CreateEvent) -> Result<Event, Error> {
        self.repo.create(event).await
    }
}