use axum::{Router, response::Html, response::IntoResponse, routing::get};
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

async fn user(
    Path(query_user): Path<String>,
    State(state): State<Arc<Vec<Post>>>,
) -> impl IntoResponse {
    let mut template = UserTemplate {
        user_name: "none",
        created_date: "none",
    };

    for i in 0..state.len() {
        if query_user == state[i].user_name {
            template = UserTemplate {
                user_name: &state[i].user_name,
                created_date: &state[i].created_date,
            };
            break;
        } else {
            continue;
        }
    }
    if &template.user_name == &"none" {
        return (StatusCode::NOT_FOUND, "404 not found").into_response();
    }

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "try again later").into_response(),
    }
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
        .route("/user/:user_name", get(user))
        .with_state(shared_state);

    let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
