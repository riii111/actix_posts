use actix_web::web;
use actix_web::{get, HttpResponse, Responder};
use tera::Tera;

use crate::controllers::posts as post_controller;

pub fn app(cfg: &mut web::ServiceConfig) {
    let tera = web::Data::new(Tera::new("templates/**/*.html").unwrap());

    cfg.app_data(tera.clone())
        .service(crate::routes::index)
        .service(
            web::scope("/api").service(
                web::scope("/posts")
                    .route("", web::get().to(post_controller::api_index))
                    .route("/{id}", web::get().to(post_controller::api_show))
                    .route("", web::post().to(post_controller::api_create)),
                // .route("", web::put().to(post_controller::api_update)),
            ),
        )
        .service(
            web::scope("/posts")
                .service(post_controller::index)
                .service(post_controller::new)
                .service(post_controller::create)
                .service(post_controller::show),
        )
        .default_service(web::to(crate::controllers::posts::not_found));
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}
