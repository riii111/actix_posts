use crate::models::posts::Post;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: i32,
    pub posted: String,
    pub sender: String,
    pub content: String,
}

static DATA_FILENAME: &str = "data.json";

pub async fn get_all() -> Result<Vec<Post>> {
    let file = fs::read_to_string(DATA_FILENAME)?;
    let mut posts: Vec<Post> = serde_json::from_str(&file)?;
    posts.sort_by(|a, b| b.posted.cmp(&a.posted));
    Ok(posts)
}

pub async fn get(id: i32) -> Result<Option<Post>> {
    let file = fs::read_to_string("data.json")?;
    let posts: Vec<Post> = serde_json::from_str(&file)?;
    Ok(posts.into_iter().find(|post| post.id == id))
}
