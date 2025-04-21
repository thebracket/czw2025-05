# For Loop

For loops in Rust operate on *ranges*. It's a lot like Python:

```python3
for i in range(0,20):
    print("{}".format(i))
```

Equivalent Rust:

```rust
fn main() {
    for i in 0..20 {
        println!("{i}");
    }
}
```

Just like Python, ranges are *exclusive* by default. So `0..20` counts from `0` to `19`. You can make in *inclusive* like this:

```rust
fn main() {
    for i in 0..=20 {
        println!("{i}");
    }
}
```

Just like Python, you can also use this to read through the contents of lists and similar---we'll deal with that when we get to lists/vectors!