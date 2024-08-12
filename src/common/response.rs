use crate::repositories::posts::Message;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum ResponseContent {
    Items(Vec<Message>),
    Item(Message),
    Reason(String),
    None,
}

#[derive(Serialize, Debug)]
pub struct ApiResponse {
    pub status: String,
    pub result: Option<ResponseContent>,
}
