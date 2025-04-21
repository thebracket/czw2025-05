# Unexpected

So we're on pretty solid ground.

```rust
fn print_it(s: String) {
    println!("{s}");
}

fn main() {
    print_it("my string".to_string());
}
```

Let's add a a step:

```rust
fn print_it(s: String) {
    println!("{s}");
}

fn main() {
    let s = "my string".to_string();
    print_it(s);
}
```

Ok - so that still prints `my string`. You'd expect that.

```rust
fn print_it(s: String) {
    println!("{s}");
}

fn main() {
    let s = "my string".to_string();
    print_it(s);
    println!("{s}");
}
```

> BOOOOOM!

It didn't compile. Anyone used to other languages would be saying "That's odd" (or more likely words I shouldn't use while teaching).

So why on Earth didn't that work?

The default pattern in Rust is to `move` variables. `print_it(s: String)` *takes ownership* of `s`, and when the function scope ends - `s` is destroyed.

That's not always what you want!