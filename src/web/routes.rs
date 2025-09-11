use crate::web::handlers::home;
use actix_web::web;

pub fn register(cfg: &mut web::ServiceConfig) {
    let scope_api =
        web::scope("/api").service(web::resource("/").route(web::get().to(home::index)));

    cfg.route("/", web::get().to(home::index))
        .service(scope_api);
}
