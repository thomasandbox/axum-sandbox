use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Request, Response, Schema};
use axum::response::IntoResponse;
use axum::{extract::Extension, handler::get, response::Html, AddExtensionLayer, Json, Router};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
  async fn add(&self, a: i32, b: i32) -> i32 {
    a + b
  }
}

type SampleSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct Sample {
  id: u32,
}

impl Sample {
  pub fn new() -> Self {
    Self { id: 1 }
  }
}

async fn graphql_handler(schema: Extension<SampleSchema>, req: Json<Request>) -> Json<Response> {
  schema.execute(req.0).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
  Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
  let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
    .data(Sample::new())
    .finish();

  let app = Router::new()
    .route("/", get(graphql_playground).post(graphql_handler))
    .layer(AddExtensionLayer::new(schema));

  println!("Playground: http://localhost:3000");

  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap()
}
