use serde::{Deserialize, Serialize};
use std::fs;

// アトリビュート.
// JSON-構造体への相互変換と、デバッグ時の構造体の表示、構造体のコピーを可能とする
// 構造体に適用しているが、ファイル全体に適用したければ"#![attribute]"と実装する
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: i32,
    pub posted: String, // 投稿日時.
    pub sender: String, // 投稿者の名前.
    pub content: String,
}

static DATA_FILENAME: &str = "data.json";

pub fn get_all() -> Vec<Message> {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let mut json_data: Vec<Message> = serde_json::from_str(&file).unwrap();
    // unwrap(): エラー発生時にパニックさせる. エラーハンドリングの簡易的実装.
    json_data.sort_by(|a, b| b.posted.cmp(&a.posted)); // |a, b| b.posted.cmp(&a.posted) はクロージャ
    json_data
}

pub fn get(id: i32) -> Message {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let json_data: Vec<Message> = serde_json::from_str(&file).unwrap();
    let mut message = Message {
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

pub fn create(mut message: Message) -> Message {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let mut json_data: Vec<Message> = serde_json::from_str(&file).unwrap();
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
