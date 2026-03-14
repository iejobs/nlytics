use actix_web::web;
use crate::config::Config;
use crate::handlers;
use crate::middleware::RequirePrivate;

pub fn routes(_config: web::Data<Config>) -> impl Fn(&mut web::ServiceConfig) {
    move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            web::scope("/api/v1/events")
                // POST — доступен с любым ключом (public или private)
                .route("", web::post().to(handlers::events::create))
                // GET — только с private ключом
                .route("", web::get().guard(RequirePrivate).to(handlers::events::get_all))
        );
    }
}