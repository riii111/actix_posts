use actix_web::web;
use actix_web::{get, HttpResponse, Responder};
use tera::Tera;

use crate::endpoints::posts;

pub fn app(cfg: &mut web::ServiceConfig) {
    let tera = web::Data::new(Tera::new("templates/**/*.html").unwrap());

    cfg.app_data(tera.clone())
        .service(crate::routes::index)
        .service(
            web::scope("/api").service(
                web::scope("/posts")
                    .route("", web::get().to(posts::api_index))
                    .route("/{id}", web::get().to(posts::api_show))
                    .route("", web::post().to(posts::api_create)),
                // .route("", web::put().to(posts::api_update)),
            ),
        )
        .default_service(web::to(crate::endpoints::posts::api_not_found))
        .service(
            web::scope("/posts")
                .service(posts::index)
                .service(posts::new)
                .service(posts::create)
                .service(posts::show),
        )
        .default_service(web::to(crate::endpoints::posts::not_found));
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}
