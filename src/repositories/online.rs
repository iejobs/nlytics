use async_trait::async_trait;
use uuid::Uuid;
use crate::models::online::GetOnline;
use crate::models::Ping;

#[async_trait]
pub trait OnlineRepository: Send + Sync {
    async fn init(&self, session_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn ping(&self, ping: Ping) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn get(&self) -> Result<GetOnline, Box<dyn std::error::Error + Send + Sync>>;
}