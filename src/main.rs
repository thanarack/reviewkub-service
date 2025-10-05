use actix_web::{App, HttpServer};
use dotenvy::dotenv;

mod db;
mod models;
mod schema;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::init_pool();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .configure(web::configure)
    })
    .bind(("127.0.0.1", 8080))?;

    println!("Server running at http://127.0.0.1:8080");

    server.run().await
}
