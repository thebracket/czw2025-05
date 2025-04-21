# Read Line as a Service

Now let's copy-paste our read_line function into the library, and make it public:

```rust
pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
```

And let's do a quick bit of setup for the next section.

1. Change directory in a terminal to the root of your workspace.
2. Create a new project named `login`.
3. Edit `Cargo.toml` to include your `authentication` library as a dependency.

> The syntax is `cargo new login`