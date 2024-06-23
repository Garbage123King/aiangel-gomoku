//use crate::run::add;
mod g5r;
mod g7r;
mod g5;
mod p5;
//#[macro_use]
mod brain;
mod base;
use base::Color;
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
   let bb = brain::create_brain(Color::Black);
   let wb = brain::create_brain(Color::White);
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
   // 将ID以逗号分割
   let parts: Vec<&str> = id.split(", ").collect();

   if parts.len() == 2 {
      // 将分割后的字符串解析为u8
      if let (Ok(x), Ok(y)) = (parts[0].parse::<u8>(), parts[1].parse::<u8>()) {
          // 存储到全局变量
         //  let mut click_event = CLICK_EVENT.lock().unwrap();
         //  *click_event = Some((x, y));
          
          // 返回点击的序号
          return Html(format!("Clicked at position ({}, {})", x, y));
      }
   }

   // 如果解析失败，返回错误信息
   Html("Invalid ID format".to_string())
}