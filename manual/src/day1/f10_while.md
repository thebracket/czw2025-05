# While

In Python, `while` loops look like this:

```python3
count = 0
while count < 5:
    print("Count:", count)
    count += 1
print("Loop finished.")
```

The Rust equivalent is basically the same:

```rust
fn main() {
    let mut count = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }
    println!("Loop finished.");
}