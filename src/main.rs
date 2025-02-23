use axum::{Router, routing::get};
use axum::{http::StatusCode, response::IntoResponse};
use tokio::net::TcpListener;

async fn index() -> impl IntoResponse {
    (StatusCode::OK, "Homepage")
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
