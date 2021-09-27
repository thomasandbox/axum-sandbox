use axum::{extract, handler::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  let app = Router::new().route("/users/:user_id/posts/:post_id", get(user_post));
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

  println!("Listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn user_post(params: extract::Path<(u32, u32)>) -> String {
  let user_id = params.0;
  let post_id = params.1;
  format!("user_id: {:?}, post_id: {:?}", user_id, post_id)
}
