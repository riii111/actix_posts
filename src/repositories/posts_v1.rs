use crate::models::posts::Post;
use std::fs;

static DATA_FILENAME: &str = "data.json";

pub fn get_all() -> Vec<Post> {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let mut json_data: Vec<Post> = serde_json::from_str(&file).unwrap();
    json_data.sort_by(|a, b| b.posted.cmp(&a.posted));
    json_data
}

pub fn get(id: i32) -> Post {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let json_data: Vec<Post> = serde_json::from_str(&file).unwrap();
    let mut message = Post {
        id: 0,
        posted: "".to_string(),
        sender: "".to_string(),
        content: "".to_string(),
    };
    if let Some(index) = json_data.iter().position(|item| item.id == id) {
        message = json_data[index].clone();
    }
    message
}

pub fn create(mut message: Post) -> Post {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let mut json_data: Vec<Post> = serde_json::from_str(&file).unwrap();
    let mut max = 0;

    for item in &json_data {
        max = std::cmp::max(item.id, max);
    }
    message.id = max + 1;
    json_data.push(message);

    let json_str = serde_json::to_string(&json_data).unwrap();
    let _ = fs::write(DATA_FILENAME, json_str);
    json_data.pop().unwrap()
}

// TODO: 編集機能.
// pub fn update(message: &Post) {
//     let file = fs::read_to_string(DATA_FILENAME).unwrap();
//     let mut json_data: Vec<Post> = serde_json::from_str(&file).unwrap();

//     if let Some(index) = json_data.iter().position(|item| item.id == message.id) {
//         json_data[index] = message.to_owned();
//     }

//     let json_str = serde_json::to_string(&json_data).unwrap();
//     let _ = fs::write(DATA_FILENAME, json_str);
// }
