// use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web::{get, HttpResponse, Responder};
use log::info;

mod data;

#[get("/posts")]
pub async fn index() -> impl Responder {
    info!("Called index");
    let posts = data::get_all();
    let mut body_str: String = "".to_string();

    body_str += include_str!("../static/header.html");
    for item in &posts {
        //  超簡易版.
        body_str += &format!("<div><a href=\"/posts/{}\">", item.id);
        body_str += &format!("<div>{} {}</div>", item.sender, item.posted);
        body_str += &format!("<div><p>{}</p></div>", item.content.replace("\n", "<br />"));
        body_str += "</a></div>";
    }
    body_str += "<div><a href=\"/posts/new\">作成</a></div>";
    body_str += include_str!("../static/footer.html");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}
