use axum::{extract::Json, routing::post, Router};
use serde::Deserialize;

// A struct for the JSON body
#[derive(Deserialize)]
struct Item {
    title: String,
}

// A handler to demonstrate the JSON body extractor
async fn add_item(Json(item): Json<Item>) -> String {
    format!("Added item: {}", item.title)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/add-item", post(add_item));
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
