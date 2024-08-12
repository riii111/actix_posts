use crate::common::response::ResponseContent;
use crate::common::response_builder::ApiResponseBuilder;
use crate::common::response_formatter::build_response;
use crate::query_params::PostQueries;
use crate::repositories::posts as post_repository;
use crate::services::posts as post_service;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Local};
use log::{error, info};

// pub async fn api_index(_req: HttpRequest, query: web::Query<PostQueries>) -> impl Responder {
//     info!("Called index API");
//     let param = query.into_inner();
//     let response = post_service::get_all_posts();
//     build_response(&param.format, &response)
// }

pub async fn api_index(_req: HttpRequest, query: web::Query<PostQueries>) -> HttpResponse {
    info!("Called index API");
    let param = query.into_inner();
    match post_service::get_all_posts().await {
        Ok(response) => build_response(&param.format, &response),
        Err(e) => {
            error!("Error in api_index: {:?}", e);
            HttpResponse::InternalServerError().json(
                ApiResponseBuilder::new()
                    .status("Error".to_string())
                    .result(ResponseContent::Reason("Internal server error".to_string()))
                    .build(),
            )
        }
    }
}

// pub async fn api_show(info: web::Path<i32>, query: web::Query<PostQueries>) -> impl Responder {
//     info!("Called show API");
//     let info = info.into_inner();
//     let param = query.into_inner();
//     let response = post_service::get_post(info);
//     build_response(&param.format, &response)
// }
pub async fn api_show(info: web::Path<i32>, query: web::Query<PostQueries>) -> HttpResponse {
    info!("Called show API");
    let id = info.into_inner();
    let param = query.into_inner();
    match post_service::get_post(id).await {
        Ok(response) => build_response(&param.format, &response),
        Err(e) => {
            error!("Error in api_show: {:?}", e);
            HttpResponse::InternalServerError().json(
                ApiResponseBuilder::new()
                    .status("Error".to_string())
                    .result(ResponseContent::Reason("Internal server error".to_string()))
                    .build(),
            )
        }
    }
}

pub async fn api_not_found() -> impl Responder {
    let response = post_service::not_found();
    HttpResponse::NotFound().json(response)
}

pub async fn api_create(params: web::Json<post_repository::Message>) -> impl Responder {
    info!("Called create API");
    let now: DateTime<Local> = Local::now();
    let message = post_repository::Message {
        id: 0,
        posted: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        sender: params.sender.clone(),
        content: params.content.clone(),
    };
    let response = post_service::create_post(message);
    let format: Option<String> = Some("json".to_string());
    build_response(&format, &response)
}

// TODO: 編集機能.

// TODO: 削除機能.
