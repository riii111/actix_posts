use actix_web::web;
use tera::Tera;

use crate::controllers::posts as post_controller;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    let tera = web::Data::new(Tera::new("templates/**/*.html").unwrap());

    cfg.app_data(tera.clone())
        .service(crate::routes::index)
        .service(
            web::scope("/posts")
                .service(post_controller::index)
                .service(post_controller::new)
                .service(post_controller::create)
                .service(post_controller::show),
        )
        .default_service(web::to(crate::controllers::posts::not_found));
}
