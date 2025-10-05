use actix_web::web;

pub mod handlers;
pub mod routes;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(routes::register);
}
