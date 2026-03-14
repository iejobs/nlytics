use actix_web::{App, HttpServer, web};
use actix_cors::Cors;
use nlytics::config::Config;
use nlytics::database;
use nlytics::routes::api;
use nlytics::repositories::postgres::PostgresEventRepository;
use nlytics::services::{EventService, OnlineService};
use nlytics::middleware::AuthMiddleware;
use std::sync::Arc;
use nlytics::repositories::RedisOnlineRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();

    let database_url = config.database_url.clone();
    let redis_url = config.redis_url.clone();
    let host = config.host.clone();
    let port = config.port;

    let config_data = web::Data::new(config);

    let pg_pool = database::postgres::init(&database_url).await;
    database::postgres::run_migrations(&pg_pool).await;

    let redis_pool = database::redis::init(&redis_url).await;

    println!("Server running at http://{}:{}", host, port);

    let event_repo = Arc::new(PostgresEventRepository::new(pg_pool.clone()));
    let event_service = web::Data::new(EventService::new(event_repo));

    let online_repo = Arc::new(RedisOnlineRepository::new(redis_pool.clone()));
    let online_service = web::Data::new(OnlineService::new(online_repo));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        // Единый AuthMiddleware на всё приложение
        let auth = AuthMiddleware::new(
            config_data.public_api_key.clone(),
            config_data.private_api_key.clone(),
            config_data.secret_key.clone(),
        );

        App::new()
            .wrap(cors)
            .wrap(auth)
            .app_data(config_data.clone())
            .app_data(event_service.clone())
            .app_data(online_service.clone())
            .configure(api::events::routes(config_data.clone()))
            .configure(api::online::routes(config_data.clone()))
    })
        .bind((host.as_str(), port))?
        .run()
        .await
}