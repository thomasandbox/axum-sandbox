use axum::{extract, handler::post, response, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  let app = Router::new().route("/", post(ser));
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

  println!("Listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap()
}

#[derive(Deserialize)]
struct Foo {
  count: i32,
}

#[derive(Serialize)]
struct Bar {
  count: i32,
}

async fn ser(req: extract::Json<Foo>) -> response::Json<Bar> {
  response::Json(Bar {
    count: req.count + 1,
  })
}
