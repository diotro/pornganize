use actix_web::web;

mod endpoints;

pub fn configure_web_app(cfg: &mut web::ServiceConfig) {
    cfg.service(endpoints::hello)
        .service(endpoints::echo)
        .route("/hey", web::get().to(endpoints::manual_hello));
}
