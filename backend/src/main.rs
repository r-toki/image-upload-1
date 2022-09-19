mod application;
mod domain;
mod infrastructure;
mod lib;
mod presentation;

use crate::lib::config::CONFIG;
use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;
use lib::module::Module;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let host = &CONFIG.host;
    let port = &CONFIG.port;
    let database_url = &CONFIG.database_url;
    let frontend_origin = &CONFIG.frontend_origin;

    let pool = PgPool::connect(database_url).await.unwrap();
    let module = Module::new(pool);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3_600);

        App::new()
            .app_data(Data::new(module.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(presentation::index::init)
            .configure(presentation::blobs::init)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
