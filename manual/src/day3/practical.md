# Practical For The Day

We're going to build a web service that works with our Authentication library from Day 1.

## Hello World Server

Make a new project.

Let's start by adding Tokio (the async framework) and Axum (webserver):

```bash
cargo add tokio -F full
cargo add axum
cargo add anyhow
cargo add serde -F derive
```

Then our `main.rs` file can be a simple webserver:

```rust
use anyhow::Result;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;
    let app = Router::new()
        .route("/", get(hello_route));
    axum::serve(listener, app).await?;
    Ok(())
}

async fn hello_route() -> &'static str {
    "Hello, world!"
}
```

## Handle Some Json

Let's add a JSON type:

```rust
#[derive(Serialize, Deserialize)]
struct HelloJson {
    message: String,
}
```

And a handler for it:

```rust
async fn hello_json_route() -> axum::Json<HelloJson> {
    let response = HelloJson {
        message: "Hello, world!".to_string(),
    };
    axum::Json(response)
}
```

Add it to the router:

```rust
    let app = Router::new()
        .route("/", get(hello_route))
        .route("/hello_json", get(hello_json_route));
```

And bingo - http://localhost:3001/hello_json displays some JSON.

## Workshop 1

### Step 1: Add your authentication library to your Cargo.toml

### Step 2: Add a Route that Lists All Users