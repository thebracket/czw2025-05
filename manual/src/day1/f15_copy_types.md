# Copy Types

And one more rule. Some types are *copy types*. These are primitives (`i8, i16, i32, i64, u8, u16...`), and never complex types like `String`.

A copy type *does not get invalidated on move*. So our original example:

```rust
fn print_it(s: i32) {
    println!("{s}");
}

fn main() {
    let s = 12;
    print_it(s);
    println!("{s}");
}
```

Works just fine for copy types. This works because copy types are typically placed into registers and not subject to `memcpy` or similar on a low-level. It's also a little surprising.