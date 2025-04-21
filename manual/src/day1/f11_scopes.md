# Scopes & Expressions

Rust *scopes* are anything enclosed in `{ .. }`. You can put scopes just about anywhere:

```rust
fn main() { // Main function score
    let x = 3;
    { // Manually created an inner scope
        let y = 4;
    }
}
```

When a scope ends, variables that were declared inside it are destroyed. There's no garbage collector, it happens immediately.