# Create a New Library

We're going to build a Rust library in our workspace, and then make use of it from other projects.

In a terminal, go to the root of the workspace we've been using. Let's create a library project:

```bash
cargo new --lib authentication
```

Cargo will have added `authentication` to your `Cargo.toml` workspace members for you. Take a quick look at the new `authentication` directory, and there's one difference: instead of `main.rs`, you have `lib.rs`. That's the only difference between making a library and an application (and you can even have both in one directory).