use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};
use serde::Deserialize;

// A struct for query parameters
#[derive(Deserialize)]
struct Page {
    number: u32,
}

// A handler to demonstrate path and query extractors
async fn show_item(Path(id): Path<u32>, Query(page): Query<Page>) -> String {
    format!("Item {} on page {}", id, page.number)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/item/:id", get(show_item));
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
