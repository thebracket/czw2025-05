# Shadowing

You can also do this:

```rust
fn main() {
    let x = 3;
    let x = x + 1; // Or x += 1
    println!("{x}");
}
```

What's happening here is called *shadowing*. The first `x` still exists, but you can't access it while the second variable bound to the same name exists. The compiler will quietly optimize away the redundancy.

This is very popular among pure-functional programmers who prefer the "bind once" style. It also often maps better when converting math papers into code. 

Both are valid.
