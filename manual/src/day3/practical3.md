# Third Practical - Web Service Client

Let's build a client for our webservice!

We'll need some dependencies:

```bash
cargo add tokio -F full
cargo add reqwest -F json
cargo add anyhow
```

We'll also add a link to our authentication crate:

```toml
[dependencies]
reqwest = { version = "0.12.15", features = ["json"] }
tokio = { version = "1.45.0", features = ["full"] }
anyhow = "1.0.98"
authentication = { path = "../../../live/authentication" }
```

## Practical: Get a single user

Ask the user on the console for a username, and try to fetch that user via the web service.

## Let's Add a POST route

Adding a user is pretty straightforward:

```rust
async fn add_user_route(
    Extension(users): Extension<Arc<Mutex<Vec<User>>>>,
    axum::extract::Json(new_user): axum::extract::Json<User>,
) -> Result<StatusCode, StatusCode> {
    let mut users = users.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    users.push(new_user);
    Ok(StatusCode::CREATED)
}
```

And in the router:

```rust
.route("/add_user", axum::routing::post(add_user_route))
```

And make a client:

```rust
use authentication::User;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let new_user = User {
        username: "fred".to_string(),
        password: "ReallySecret".to_string(),
        role: authentication::LoginRole::Admin,
    };

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3001/add_user")
        .json(&new_user)
        .send()
        .await?;
    println!("Response: {:?}", res);
    Ok(())
}
```

Now go to http://localhost:3001/all_users - and the new user is there.

> See if you can extend this to delete a user