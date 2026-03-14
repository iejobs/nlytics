use actix_web::{web, HttpResponse, Error};
use crate::services::EventService;
use crate::models::event::{CreateEvent, EventResponse};

pub async fn get_all(
    service: web::Data<EventService>,
) -> HttpResponse {
    match service.get_all().await {
        Ok(events) => {
            let response: Vec<EventResponse> = events.into_iter().map(EventResponse::from).collect();
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            eprintln!("Failed to fetch events: {:?}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn create(
    service: web::Data<EventService>,
    payload: web::Json<CreateEvent>,
) -> Result<HttpResponse, Error> {
    let event = service
        .create(payload.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let response = EventResponse::from(event);
    Ok(HttpResponse::Created().json(response))
}