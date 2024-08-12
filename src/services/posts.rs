use crate::common::response::{ApiResponse, ResponseContent};
use crate::repositories::posts as post_repository;
use anyhow::Result;

pub async fn get_all_posts() -> Result<ApiResponse> {
    let posts = post_repository::get_all().await?;
    Ok(ApiResponse::builder()
        .status("OK".to_string())
        .result(ResponseContent::Items(posts))
        .build())
}

pub async fn get_post(id: i32) -> Result<ApiResponse> {
    let post = post_repository::get(id).await?;
    match post {
        Some(post) => Ok(ApiResponse::builder()
            .status("OK".to_string())
            .result(ResponseContent::Item(post))
            .build()),
        None => Ok(ApiResponse::builder()
            .status("Not Found".to_string())
            .result(ResponseContent::Reason("Post not found".to_string()))
            .build()),
    }
}

pub fn get_post(id: i32) -> ApiResponse {
    let post = post_repository::get(id);
    ApiResponse::builder()
        .status("OK".to_string())
        .result(ResponseContent::Item(post))
        .build()
}

pub fn create_post(message: post_repository::Message) -> ApiResponse {
    let created_message = post_repository::create(message);
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
