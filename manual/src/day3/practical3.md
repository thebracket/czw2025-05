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

