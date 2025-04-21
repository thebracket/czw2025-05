# Scopes & Shadowing

Scopes and shadowing work as a great way to illustrate scope variable destruction:

```rust
fn main() {
    let x = 3;
    {
        let x = 4;
        println!("{x}");
    }
    println!("{x}");
}
```

When the inner scope ends, it's shadowed X is destroyed --- and the binding reverts to the original.