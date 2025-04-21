# Simple Function

Let's make a really simple function:

```rust
fn print_it(s: String) {
    println!("{s}");
}

fn main() {
    print_it("my string".to_string());
}
```

> Why the `to_string`? Rust strings are special. "hello" is a type `&str` - pointer to a set of characters in memory. Calling `to_string()` turns it into a full `String` type, designed for appending, calculations, and use as a buffer. We'll talk about this later. For now, we'll be happy that it's a string!

Anyway, you can probably guess what this does. It prints `my string`.