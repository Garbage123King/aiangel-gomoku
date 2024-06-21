//use crate::run::add;
mod gr5;
mod gr7;
//#[macro_use]
mod brain;
use brain::Color;
use pulldown_cmark::{html, Options, Parser};
// fn main() {
//     let bb = brain::create_brain(Color::Black);
//     let wb = brain::create_brain(Color::White);
// }
use std::{env, fs, net::SocketAddr};
use axum::{
   response::{Html, IntoResponse},
   routing::{get, delete},
   Router,
   extract::Path
};
 
 #[tokio::main]
 async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/doc", get(doc))
        .route("/click/:id", get(click)); // 添加处理点击事件的路由
 
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}

async fn root() -> impl IntoResponse {
   let static_dir = format!("{}/static", env!("CARGO_MANIFEST_DIR"));

   match fs::read_to_string(format!("{}/index.html", static_dir)) {
       Ok(content) => Html(content),
       Err(e) => Html(format!("Error : {}", e)),
   }
}

async fn doc() -> impl IntoResponse {
   let static_dir = format!("{}/static", env!("CARGO_MANIFEST_DIR"));

   // 读取 Markdown 文件内容
   let markdown_content = match fs::read_to_string(format!("{}/doc.md", static_dir)) {
      Ok(content) => content,
      Err(e) => return Html(format!("Error : {}", e)),
   };

   // 使用 pulldown-cmark 将 Markdown 转换为 HTML
   let mut html_output = String::new();
   let parser = Parser::new_ext(&markdown_content, Options::empty());
   html::push_html(&mut html_output, parser);

   Html(html_output)
}

async fn click(Path(id): Path<String>) -> impl IntoResponse {
   // 使用全局变量存储点击事件
   println!("{}",id);

   // 返回点击的序号
   Html(format!("Clicked at position ({}, {})", 6, 6))
}