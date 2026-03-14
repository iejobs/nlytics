use crate::models::event::{Event, CreateEvent};
use crate::repositories::EventRepository;
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::Error;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostgresEventRepository {
    pool: PgPool,
}

impl PostgresEventRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EventRepository for PostgresEventRepository {
    async fn get_all(&self) -> Result<Vec<Event>, Error> {
        sqlx::query_as::<_, Event>("SELECT * FROM events ORDER BY id DESC")
            .fetch_all(&self.pool)
            .await
    }

    async fn create(&self, event: CreateEvent) -> Result<Event, Error> {
        let created_event = sqlx::query_as::<_, Event>(
            r#"
            INSERT INTO events (id, "type", data)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
            .bind(Uuid::new_v4())
            .bind(&event.r#type)
            .bind(&event.data)
            .fetch_one(&self.pool)
            .await?;
        Ok(created_event)
    }
}