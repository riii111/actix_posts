use actix_web::{get, post, web, HttpResponse, Responder};
// use chrono::{DateTime, Duration, Local};
use chrono::{DateTime, Local};
use log::info;
// use serde::{Deserialize, Serialize};
use serde::Deserialize;
use tera::Context;

mod data;

#[get("/posts")]
pub async fn index(tmpl: web::Data<tera::Tera>) -> impl Responder {
    info!("Called index");
    let posts = data::get_all();
    let mut context = Context::new();

    context.insert("posts", &posts);
    let body_str = tmpl.render("index.html", &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[get("/posts/{id}")]
pub async fn show(tmpl: web::Data<tera::Tera>, info: web::Path<i32>) -> impl Responder {
    // Path<i32> = パスパラメータを受け取るための構造体
    info!("Called show");
    let info = info.into_inner();
    let post = data::get(info);
    let mut context = Context::new();

    context.insert("post", &post);
    let body_str: String = tmpl.render("show.html", &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Page not found!")
}

#[get("/posts/new")]
pub async fn new(tmpl: web::Data<tera::Tera>) -> impl Responder {
    info!("Called new");
    let mut context = Context::new();
    let post = data::Message {
        id: 0,
        sender: "".to_string(),
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

#[post("/posts/create")]
pub async fn create(params: web::Form<CreateForm>) -> impl Responder {
    let now: DateTime<Local> = Local::now();
    let mut message = data::Message {
        id: 0,
        posted: now.format("%Y-%m-%d%H:%M:%S").to_string(),
        sender: params.sender.clone(),
        content: params.content.clone(),
    };
    message = data::create(message);

    web::Redirect::to(format!("/posts/{}", message.id)).see_other()
}
