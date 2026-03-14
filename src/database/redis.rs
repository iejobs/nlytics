use deadpool_redis::{Config, Runtime, Pool};

pub type RedisPool = Pool;

pub async fn init(redis_url: &str) -> RedisPool {
    let cfg = Config::from_url(redis_url);

    let pool = cfg.create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create Redis pool object");

    let _conn = pool.get().await.expect("Failed to connect to Redis");

    pool
}