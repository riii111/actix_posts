use actix_web::{get, post, web, HttpResponse, Responder};
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

#[get("/posts/{id}")]
pub async fn show(info: web::Path<i32>) -> impl Responder {
    // Path<i32> = パスパラメータを受け取るための構造体
    info!("Called show");
    let info = info.into_inner();
    let post = data::get(info);
    let mut body_str: String = "".to_string();

    body_str += include_str!("../static/header.html");
    body_str += "<div>";

    if post.id != 0 {
        body_str += &format!("<div>投稿者：{}</div>", post.sender);
        body_str += &format!("<div>投稿日時：{}</div>", post.posted);
        body_str += &format!(
            "<div>投稿内容：<br />{}</div>",
            post.content.replace("\n", "<br />")
        );
        body_str += &format!("<div><a href=\"/posts/{}/edit\">編集</a> ", info);
        body_str += &format!("<a href=\"/posts/{}/delete\">削除</a><div>", info);
    } else {
        body_str += "見つかりません。";
    }
    body_str += "</div>";
    body_str += "<div><a href=\"/posts\">一覧へ</a></div>";
    body_str += include_str!("../static/footer.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Page not found!")
}
