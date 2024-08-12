/* コンテンツを含めて返すposts */

use crate::models::posts::Post;
use crate::repositories::posts_v1 as post_v1_repository;
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages, Level};
use chrono::{DateTime, Local};
use log::info;
use serde::Deserialize;
use tera::Context;

#[get("")]
pub async fn index(tmpl: web::Data<tera::Tera>, messages: IncomingFlashMessages) -> impl Responder {
    info!("Called index");
    let posts = post_v1_repository::get_all();
    let mut context = Context::new();

    for message in messages.iter() {
        match message.level() {
            Level::Success => context.insert("success", &message.content()),
            Level::Error => context.insert("error", &message.content()),
            _ => (),
        }
    }
    context.insert("posts", &posts);
    let body_str = tmpl.render("index.html", &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[get("/{id}")]
pub async fn show(
    tmpl: web::Data<tera::Tera>,
    info: web::Path<i32>,
    messages: IncomingFlashMessages,
) -> impl Responder {
    info!("Called show");
    let info = info.into_inner();
    let post = post_v1_repository::get(info);
    let mut context = Context::new();
    for message in messages.iter() {
        match message.level() {
            Level::Success => context.insert("success", &message.content()),
            Level::Error => context.insert("error", &message.content()),
            _ => (),
        }
    }
    context.insert("post", &post);
    let body_str: String = tmpl.render("show.html", &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Page not found!")
}

#[get("/new")]
pub async fn new(tmpl: web::Data<tera::Tera>, session: Session) -> impl Responder {
    info!("Called new");
    let mut context = Context::new();
    let sender = session
        .get::<String>("sender")
        .unwrap()
        .unwrap_or_else(|| "名無しさん".to_string());
    let post = Post {
        id: 0,
        sender: sender,
        content: "".to_string(),
        posted: "".to_string(),
    };
    context.insert("action", "create");
    context.insert("post", &post);
    context.insert("button", "投稿");

    let body_str: String = tmpl.render("form.html", &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[derive(Deserialize, Debug)]
pub struct CreateForm {
    id: i32,
    posted: String,
    sender: String,
    content: String,
}

#[post("/create")]
pub async fn create(params: web::Form<CreateForm>, session: Session) -> impl Responder {
    info!("Called create");
    let now: DateTime<Local> = Local::now();
    let mut message = Post {
        id: 0,
        posted: now.format("%Y-%m-%d%H:%M:%S").to_string(),
        sender: params.sender.clone(),
        content: params.content.clone(),
    };
    message = post_v1_repository::create(message);
    if message.id == 0 {
        FlashMessage::error("投稿でエラーが発生しました").send();
    } else {
        FlashMessage::success("投稿しました").send();
    }
    let _ = session.insert("sender", params.sender.clone());
    web::Redirect::to(format!("/posts/{}", message.id)).see_other()
}

// TODO: 編集機能.
// #[put("/update/{id}")]
// pub async fn update(
//     params: web::Form<CreateForm>,
//     info: web::Path<i32>,
//     session: Session,
// ) -> impl Responder {
//     info!("Called update");
//     let info = info.into_inner();
//     let mut message = post_repository::get(info);

//     if !params.sender.is_empty() {
//         message.sender = params.sender.clone();
//     }
//     if !params.content.is_empty() {
//         message.content = params.content.clone();
//     }

//     message.posted = Local::now().format("%Y-%m-%d%H:%M:%S").to_string();

//     post_repository::update(&message);

//     FlashMessage::success("更新しました").send();

//     let _ = session.insert("sender", params.sender.clone());
//     web::Redirect::to(format!("/posts/{}", message.id)).see_other()
// }
