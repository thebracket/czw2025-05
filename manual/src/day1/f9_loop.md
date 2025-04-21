# Loop

Rust includes a "keep going until we stop" loop called `loop`:

```rust
fn main() {
    let mut n = 0;
    println!("Looping 10 times");
    loop {
        n += 1;
        if n > 10 {
            break;
        }
    }
    println!("Escaped");
}
```