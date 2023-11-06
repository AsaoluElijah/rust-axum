use axum::{
    body::Body,
    http::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router, Server,
};

async fn logging_middleware(req: Request<Body>, next: Next<Body>) -> Response {
    println!("Received a request to {}", req.uri());
    next.run(req).await
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .layer(middleware::from_fn(logging_middleware));

    Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
