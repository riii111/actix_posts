use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: i32,
    pub posted: String,
    pub sender: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: i32,
    pub posted: String,
    pub sender: String,
    pub content: String,
}
