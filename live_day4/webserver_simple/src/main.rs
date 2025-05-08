use axum::routing::get;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3001").await?;
    let app = axum::Router::new()
        .route("/", get(hello_route));
    axum::serve(listener, app).await?;

    Ok(())
}

async fn hello_route() -> String {
    "Hello".to_string()
}
