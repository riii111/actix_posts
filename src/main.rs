use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use std::io::Result;

mod handler;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| App::new().service(handler::index).wrap(Logger::default()))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
