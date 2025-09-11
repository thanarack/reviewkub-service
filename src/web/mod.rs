use actix_web::web;

pub mod routes;
pub mod handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(routes::register);
}