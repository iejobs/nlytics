use async_trait::async_trait;
use crate::repositories::OnlineRepository;
use deadpool_redis::{Pool};
use redis::AsyncCommands;
use uuid::Uuid;
use crate::models::online::GetOnline;
use crate::models::Ping;

#[derive(Clone)]
pub struct RedisOnlineRepository {
    pool: Pool,
}

impl RedisOnlineRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OnlineRepository for RedisOnlineRepository {
    async fn init(&self, session_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut con = self.pool.get().await?;

        let key = format!("session:{}", session_id);
        con.set_ex::<_, _, ()>(key, "true", 60).await?;

        Ok(())
    }

    async fn ping(&self, ping: Ping) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut con = self.pool.get().await?;

        let key = format!("session:{}", ping.session.id);
        con.set_ex::<_, _, ()>(key, "true", 60).await?;

        Ok(())
    }

    async fn get(&self) -> Result<GetOnline, Box<dyn std::error::Error + Send + Sync>> {
        let mut con = self.pool.get().await?;

        let keys: Vec<String> = con.keys("session:*").await?;

        Ok(GetOnline {
            current: keys.len(),
        })
    }
}