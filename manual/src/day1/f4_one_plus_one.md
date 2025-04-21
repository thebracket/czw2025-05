# One Plus One

In Python, you're probably pretty used to this:

```python3
x = 3
x = x + 1
print("{}".format(x))
```

> It prints "4", like you'd expect.

Let's try this in Rust:

```rust
fn main() {
    let x = 3;
    x = x + 1; // Or x += 1
    println!("{x}");
}
```

That *does not compile*. Variable assignments in Rust are *immutable by default*.

> Don't give up on Rust just now. It really can add one, I promise.