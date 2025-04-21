# Returning and Move

If you like the really functional style with everything immutable, you can always *return* the value from the function:

```rust
fn print_it(s: String) -> String {
    println!("{s}");
    s // Return just by "falling out" of the expression/function/scope
}

fn main() {
    let s = "my string".to_string();
    let s = print_it(s);
    println!("{s}");
}
```

This compiles, and on a release build (`cargo run --release`) it will be optimized to do what you expect. Some people like this style, some find it clunky.

From a performance perspective, unoptimized this is copying the string into the function, working with it, and copying it back out. Without optimizations, this is a potentially slow way to work!