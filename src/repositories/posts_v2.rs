use crate::models::posts::Post;
use anyhow::Result;
use std::fs;

static DATA_FILENAME: &str = "data.json";

pub async fn get_all() -> Result<Vec<Post>> {
    let file = fs::read_to_string(DATA_FILENAME)?;
    let mut posts: Vec<Post> = serde_json::from_str(&file)?;
    posts.sort_by(|a, b| b.posted.cmp(&a.posted));
    Ok(posts)
}

pub async fn get(id: i32) -> Result<Option<Post>> {
    let file = fs::read_to_string(DATA_FILENAME)?;
    let posts: Vec<Post> = serde_json::from_str(&file)?;
    Ok(posts.into_iter().find(|post| post.id == id))
}

pub async fn create(mut post: Post) -> Result<Post> {
    let file = fs::read_to_string(DATA_FILENAME)?;
    let mut posts: Vec<Post> = serde_json::from_str(&file)?;

    let max_id = posts.iter().map(|p| p.id).max().unwrap_or(0);
    post.id = max_id + 1;
    posts.push(post.clone());

    let json_str = serde_json::to_string(&posts).context("Failed to serialize posts to JSON")?;
    fs::write(DATA_FILENAME, json_str)
        .await
        .context("Failed to write data to file")?;

    Ok(post)
}
