mod application;
mod domain;
mod infrastructure;
mod lib;
mod presentation;

use crate::lib::config::CONFIG;
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let host = &CONFIG.host;
    let port = &CONFIG.port;
    let database_url = &CONFIG.database_url;
    let frontend_origin = &CONFIG.frontend_origin;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec!["AUTHORIZATION", "CONTENT_TYPE"])
            .max_age(3_600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .configure(presentation::index::init)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
