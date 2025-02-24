use axum::{Router, routing::get};
use axum::{http::StatusCode, response::IntoResponse};
use tokio::net::TcpListener;

use sqlx::FromRow;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::time::Date;

use std::sync::Arc;

#[derive(FromRow, Debug, Clone)]
pub struct User {
    pub user_name: String,
    pub password: String,
    pub created_date: Date,
}

async fn index() -> impl IntoResponse {
    (StatusCode::OK, "Homepage")
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:admin@localhost/ifrit-db")
        .await
        .expect("couldn't connect to the database");

    let users = sqlx::query_as::<_, User>("select user_name from users")
        .fetch_all(&pool)
        .await
        .unwrap();

    let shared_state = Arc::new(users);

    let app = Router::new()
        .route("/", get(index))
        .route("/users", get(users))
        .with_state(shared_state);

    let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
