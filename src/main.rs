use actix_web::cookie::Key;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use std::io::Result;

mod configs;
mod controllers;
mod middleware;
mod models;
mod routes;

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let key = Key::generate();
    let message_framework = middleware::session::build_flash_messages_framework();

    HttpServer::new(move || {
        App::new()
            .configure(configs::app::config_app)
            .wrap(Logger::default())
            .wrap(message_framework.clone())
            .wrap(middleware::session::build_cookie_session_middleware(
                key.clone(),
            ))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
