# Second Practical - Users in an Arc

Now we can make Arc useful. Axum supports "extension layers" that can be automatically injected into routes. They are "cloned" on the way in - making `Arc` the natural way to store them:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let users = get_users();
    let shared_users = Arc::new(Mutex::new(users));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;
    let app = Router::new()
        .route("/", get(hello_route))
        .route("/hello_json", get(hello_json_route))
        .route("/all_users", get(all_users_route))
        // We've added an Extension
        .layer(Extension(shared_users));
    axum::serve(listener, app).await?;
    Ok(())
}
```

Now our users route can just access the shared data:

```rust
async fn all_users_route(
    Extension(users): Extension<Arc<Mutex<Vec<User>>>>
) -> Result<axum::Json<Vec<User>>, StatusCode> {
    // Don't forget to add Clone to the user struct!
    let users = users.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(users.clone()))
}
```

## Workshop: Add a single user get

Add a route (mapped to `/user/{name}`) that returns a single user.

HINTS:

You can add URL matches like this:

```rust
.route("/user/{username}", get(one_user_route))
```

And you can receive them as a parameter:

```rust
axum::extract::Path(username): axum::extract::Path<String>,
```