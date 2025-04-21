# Consume Hello World

Now we return to the `helloworld` project (`c:\users\herbert\rust\live\login` in my case).

Open `Cargo.toml`, and let's add our project as a dependency:

```toml
[dependencies]
authentication = { path = "../authentication" }
```

> You can specify *version numbers* (e.g. `library = "1"` using semantic versioning. Provide as many significant numbers as you need; "1" will load the latest in the "1.x.y" tree. You can also use `{ git = "git path" }` and load directly from a shared Git repository. Useful if you are working with a team, or want to grab some code from Github.)

We'll update `main.rs` to use our new function:

```rust
use authentication::greet_user;

fn main() {
    println!("{}", greet_user("Herbert"));
}
```

Run it with `cargo run`:

```
   Compiling hello_service_exe v0.1.0 (C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\src\hello_service_exe)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\target\debug\hello_service_exe.exe`
Hello Herbert
```

*Congratulations, you've made and used your first library.*