# Mutability

The easiest way to make this work is to make `x` mutable:

```rust
fn main() {
    let mut x = 3;
    x = x + 1; // Or x += 1
    println!("{x}");
}
```

So why not just declare everything `mut`? You can if you really want to (although the linter will argue with you). Immutable by default helps avoid bugs, particularly times where you change something and didn't intend to. Marking mutable variables helps future you when you come back to debug the program understand which parts of the process *might change*.