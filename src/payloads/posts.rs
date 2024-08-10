use crate::repository::posts as post_model;
use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum ResponseContent {
    Items(Vec<post_model::Message>),
    Item(post_model::Message),
    Reason(String),
    None,
}

#[derive(Serialize, Debug)]
pub struct ApiResponse {
    status: String,
    result: Option<ResponseContent>,
}

impl ApiResponse {
    pub fn builder() -> ApiResponseBuilder {
        ApiResponseBuilder::default()
    }
}

#[derive(Default)]
pub struct ApiResponseBuilder {
    status: String,
    result: Option<ResponseContent>,
}

impl ApiResponseBuilder {
    pub fn status(mut self, status: String) -> Self {
        self.status = status;
        self
    }

    pub fn result(mut self, result: ResponseContent) -> Self {
        self.result = Some(result);
        self
    }

    pub fn build(self) -> ApiResponse {
        ApiResponse {
            status: self.status,
            result: self.result,
        }
    }
}

pub fn build_response(format: &Option<String>, response: &ApiResponse) -> impl Responder {
    if let Some(format) = format {
        match format.as_str() {
            // "xml" => HttpResponse::Ok()
            //     .content_type("application/xml; charset=utf-8")
            //     .body(serde_xml_rs::to_string(&response).unwrap()),
            _ => HttpResponse::Ok().json(response),
        }
    } else {
        HttpResponse::Ok().json(response)
    }
}
