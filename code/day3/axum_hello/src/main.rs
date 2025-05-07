use anyhow::Result;
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;
    let app = Router::new()
        .route("/", get(hello_route))
        .route("/hello_json", get(hello_json_route));
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