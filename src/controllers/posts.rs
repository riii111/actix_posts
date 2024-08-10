use crate::payloads::posts::{build_response, ApiResponse, ResponseContent};
use crate::query_params::PostQueries;
use crate::repository::posts as post_model;
use actix_session::Session;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages, Level};
use chrono::{DateTime, Local};
use log::info;
use serde::Deserialize;
use tera::Context;

/*
<name>      : フルスタック、コンテンツ含めて返す
api_<name>  : APIとして提供する（画面を作成予定）
 */

#[get("")]
pub async fn index(tmpl: web::Data<tera::Tera>, messages: IncomingFlashMessages) -> impl Responder {
    info!("Called index");
    let posts = post_model::get_all();
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

// APIとして実装
pub async fn api_index(_req: HttpRequest, query: web::Query<PostQueries>) -> impl Responder {
    info!("Called index API");
    let param = query.into_inner();
    let posts = post_model::get_all();
    let response = ApiResponse::builder()
        .status("OK".to_string())
        .result(ResponseContent::Items(posts))
        .build();
    build_response(&param.format, &response)
}

#[get("/{id}")]
pub async fn show(
    tmpl: web::Data<tera::Tera>,
    info: web::Path<i32>,
    messages: IncomingFlashMessages,
) -> impl Responder {
    info!("Called show");
    let info = info.into_inner();
    let post = post_model::get(info);
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

pub async fn api_show(info: web::Path<i32>, query: web::Query<PostQueries>) -> impl Responder {
    info!("Called show API");
    let info = info.into_inner();
    let param = query.into_inner();
    let post = post_model::get(info);
    let response = ApiResponse::builder()
        .status("OK".to_string())
        .result(ResponseContent::Item(post))
        .build();
    build_response(&param.format, &response)
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Page not found!")
}

pub async fn api_not_found() -> impl Responder {
    HttpResponse::NotFound()
}

#[get("/new")]
pub async fn new(tmpl: web::Data<tera::Tera>, session: Session) -> impl Responder {
    info!("Called new");
    let mut context = Context::new();
    let sender = session
        .get::<String>("sender")
        .unwrap()
        .unwrap_or_else(|| "名無しさん".to_string());
    let post = post_model::Message {
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
    let mut message = post_model::Message {
        id: 0,
        posted: now.format("%Y-%m-%d%H:%M:%S").to_string(),
        sender: params.sender.clone(),
        content: params.content.clone(),
    };
    message = post_model::create(message);
    if message.id == 0 {
        FlashMessage::error("投稿でエラーが発生しました").send();
    } else {
        FlashMessage::success("投稿しました").send();
    }
    let _ = session.insert("sender", params.sender.clone());
    web::Redirect::to(format!("/posts/{}", message.id)).see_other()
}

pub async fn api_create(params: web::Json<post_model::Message>) -> impl Responder {
    info!("Called create API");
    let now: DateTime<Local> = Local::now();
    let mut message = post_model::Message {
        id: 0,
        posted: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        sender: params.sender.clone(),
        content: params.content.clone(),
    };
    message = post_model::create(message);
    let response = ApiResponse::builder()
        .status("OK".to_string())
        .result(ResponseContent::Item(message))
        .build();
    let format: Option<String> = Some("json".to_string());
    build_response(&format, &response)
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
//     let mut message = post_model::get(info);

//     if !params.sender.is_empty() {
//         message.sender = params.sender.clone();
//     }
//     if !params.content.is_empty() {
//         message.content = params.content.clone();
//     }

//     message.posted = Local::now().format("%Y-%m-%d%H:%M:%S").to_string();

//     post_model::update(&message);

//     FlashMessage::success("更新しました").send();

//     let _ = session.insert("sender", params.sender.clone());
//     web::Redirect::to(format!("/posts/{}", message.id)).see_other()
// }

// TODO: 削除機能.
