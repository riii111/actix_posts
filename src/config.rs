use actix_web::web;
use tera::Tera;

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    let tera = Tera::new("templates/**/*.html").unwrap();
    cfg.app_data(web::Data::new(tera));
    cfg.service(crate::handler::index);
    cfg.service(crate::handler::new);
    cfg.service(crate::handler::create);
    cfg.service(crate::handler::show);
    cfg.default_service(web::to(crate::handler::not_found));
}
