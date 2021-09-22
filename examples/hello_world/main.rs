use axum::{handler::get, response::Html, Router};
use std::net::SocketAddr;

async fn hello_world() -> Html<&'static str> {
  Html("Hello, World\n")
}

#[tokio::main]
async fn main() {
  let app = Router::new().route("/", get(hello_world));
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  println!("Listening on {}", addr);

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
