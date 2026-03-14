use actix_web::web;
use crate::config::Config;
use crate::handlers;

pub fn routes(_config: web::Data<Config>) -> impl Fn(&mut web::ServiceConfig) {
    move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            web::scope("/api/v1/online")
                // GET — только с private ключом (смотреть кто онлайн — приватно)
                .route("", web::get().guard(crate::middleware::RequirePrivate).to(handlers::online::get))
                // POST — доступен с любым ключом
                .route("/init", web::post().to(handlers::online::init))
                .route("/ping", web::post().to(handlers::online::ping))
        );
    }
}