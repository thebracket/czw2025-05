use std::sync::{Arc, Mutex};

use authentication::{get_users, User};
use axum::{http::StatusCode, routing::get, Extension, Router};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let users = get_users();
    let shared_users = Arc::new(Mutex::new(users));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;
    let app = Router::new()
        .route("/", get(hello_route))
        .route("/hello_json", get(hello_json_route))
        .route("/all_users", get(all_users_route))
        .route("/user/{username}", get(one_user_route))
        .route("/add_user", axum::routing::post(add_user_route))
        .layer(Extension(shared_users));
    axum::serve(listener, app).await?;
    Ok(())
}

async fn hello_route() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize, Deserialize)]
struct HelloJson {
    message: String,
}

async fn hello_json_route() -> axum::Json<HelloJson> {
    let response = HelloJson {
        message: "Hello, world!".to_string(),
    };
    axum::Json(response)
}

async fn all_users_route(
    Extension(users): Extension<Arc<Mutex<Vec<User>>>>
) -> Result<axum::Json<Vec<User>>, StatusCode> {
    // Don't forget to add Clone to the user struct!
    let users = users.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(users.clone()))
}

async fn one_user_route(
    Extension(users): Extension<Arc<Mutex<Vec<User>>>>,
    axum::extract::Path(username): axum::extract::Path<String>,
) -> Result<axum::Json<User>, StatusCode> {
    let users = users.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let user = users
        .iter()
        .find(|user| user.username == username)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(axum::Json(user))
}

async fn add_user_route(
    Extension(users): Extension<Arc<Mutex<Vec<User>>>>,
    axum::extract::Json(new_user): axum::extract::Json<User>,
) -> Result<StatusCode, StatusCode> {
    let mut users = users.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    users.push(new_user);
    Ok(StatusCode::CREATED)
}
