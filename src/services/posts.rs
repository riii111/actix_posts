use crate::common::response::{ApiResponse, ResponseContent};
use crate::repositories::posts as post_model;

pub fn get_all_posts() -> ApiResponse {
    let posts = post_model::get_all();
    ApiResponse::builder()
        .status("OK".to_string())
        .result(ResponseContent::Items(posts))
        .build()
}

pub fn get_post(id: i32) -> ApiResponse {
    let post = post_model::get(id);
    ApiResponse::builder()
        .status("OK".to_string())
        .result(ResponseContent::Item(post))
        .build()
}

pub fn create_post(message: post_model::Message) -> ApiResponse {
    let created_message = post_model::create(message);
    ApiResponse::builder()
        .status("OK".to_string())
        .result(ResponseContent::Item(created_message))
        .build()
}

pub fn not_found() -> ApiResponse {
    ApiResponse::builder()
        .status("NotFound".to_string())
        .result(ResponseContent::None)
        .build()
}
