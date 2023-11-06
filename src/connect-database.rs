use axum::{extract::Extension, response::IntoResponse, routing::get, Json, Router, Server};
use serde_json::json;
use sqlx::{MySqlPool, Row};

// Define the get_users function as before
async fn get_users(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
    let rows = match sqlx::query("SELECT id, name, email FROM user")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let users: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.try_get::<i32, _>("id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "email": row.try_get::<String, _>("email").unwrap_or_default(),
            })
        })
        .collect();

    (axum::http::StatusCode::OK, Json(users)).into_response()
}

#[tokio::main]
async fn main() {
    // Set up the database connection pool
    let database_url = "mysql://root:password@localhost/UserDatabase";
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Could not connect to the database");

    // Create the Axum router
    let app = Router::new()
        .route("/users", get(get_users))
        .layer(Extension(pool));

    // Run the Axum server
    Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
