use std::sync::{Arc, Mutex};
use authentication::{get_users, User};
use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

type UserList  = Arc<Mutex<Vec<User>>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let users = get_users();
    let shared_users = Arc::new(Mutex::new(users));

    let listener = TcpListener::bind("127.0.0.1:3001").await?;
    let app = axum::Router::new()
        .route("/", get(hello_route))
        .route("/hello_json", get(hello_json_route))
        .route("/all_users", get(all_users_route))
        .route("/user/{username}", get(one_user_route))
        .route("/add_user", post(add_user_route))
        .layer(Extension(shared_users));
    axum::serve(listener, app).await?;

    Ok(())
}

async fn add_user_route(
    Extension(users): Extension<Arc<Mutex<Vec<User>>>>,
    axum::extract::Json(new_user): axum::extract::Json<User>,
) -> Result<StatusCode, StatusCode> {
    let mut users = users.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    users.push(new_user);
    Ok(StatusCode::CREATED)
}

async fn one_user_route(
    Extension(users): Extension<Arc<Mutex<Vec<User>>>>,
    Path(username): axum::extract::Path<String>,
) -> Result<axum::Json<User>, StatusCode> {
    let users = users.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let user = users
        .iter()
        .find(|user| user.username == username)
        .map(|u| u.clone())
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(axum::Json(user))
}

async fn hello_route() -> String {
    "Hello".to_string()
}

#[derive(Serialize, Deserialize)]
pub struct HelloJson {
    message: String,
}

async fn hello_json_route() -> axum::Json<HelloJson> {
    let response = HelloJson {
        message: "Hello, World!".to_string()
    };
    axum::Json(response)
}

async fn all_users_route(
    Extension(users): Extension<UserList>
) -> axum::Json<Vec<User>> {
    let users = users.lock().unwrap();
    axum::Json(users.clone())
}