use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;
use crate::models::online::{GetOnline, Init};
use crate::models::Ping;
use crate::repositories::OnlineRepository;

pub struct OnlineService {
    repo: Arc<dyn OnlineRepository>,
}

impl OnlineService {
    pub fn new(repo: Arc<dyn OnlineRepository>) -> Self {
        Self { repo }
    }

    pub async fn init(&self) -> Result<Init, Box<dyn Error + Send + Sync>> {
        let session_id = Uuid::new_v4();
        self.repo.init(session_id).await?;
        Ok(Init {
            session_id: session_id.to_string(),
        })
    }

    pub async fn ping(&self, ping: Ping) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.repo.ping(ping).await
    }

    pub async fn get(&self) -> Result<GetOnline, Box<dyn Error + Send + Sync>> {
        self.repo.get().await
    }
}