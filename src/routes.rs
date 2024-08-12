use actix_web::web;
use actix_web::{get, HttpResponse, Responder};
use tera::Tera;

use crate::controllers::posts_v1;
use crate::controllers::posts_v2;

pub fn app(cfg: &mut web::ServiceConfig) {
    let tera = web::Data::new(Tera::new("templates/**/*.html").unwrap());

    cfg.app_data(tera.clone())
        .service(crate::routes::index)
        .service(
            web::scope("/api").service(
                web::scope("/posts")
                    .route("", web::get().to(posts_v2::api_index))
                    .route("/{id}", web::get().to(posts_v2::api_show))
                    .route("", web::post().to(posts_v2::api_create)),
                // .route("", web::put().to(posts::api_update)),
            ),
        )
        .default_service(web::to(crate::controllers::posts_v2::api_not_found))
        .service(
            web::scope("/posts")
                .service(posts_v1::index)
                .service(posts_v1::new)
                .service(posts_v1::create)
                .service(posts_v1::show),
        )
        .default_service(web::to(crate::controllers::posts_v1::not_found));
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}
