use actix_web::{web, HttpResponse};
use crate::models::Ping;
use crate::services::OnlineService;

pub async fn init(
    service: web::Data<OnlineService>
) -> HttpResponse {
    match service.init().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn ping(
    service: web::Data<OnlineService>,
    payload: web::Json<Ping>,
) -> HttpResponse {
    let _ = service.ping(payload.into_inner()).await;

    HttpResponse::Ok().finish()
}

pub async fn get(
    service: web::Data<OnlineService>,
) -> HttpResponse {
    match service.get().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}