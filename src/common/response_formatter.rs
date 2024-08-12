use crate::common::response::ApiResponse;
use actix_web::{HttpResponse, Responder};

pub fn build_response(format: &Option<String>, response: &ApiResponse) -> impl Responder {
    match format.as_deref() {
        Some("xml") => unimplemented!("XML format is not supported yet"),
        _ => HttpResponse::Ok().json(response),
    }
}
