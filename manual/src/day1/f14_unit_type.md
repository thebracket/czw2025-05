# The Unit Type

What do you think this does? (Hint: `:?` is 'debug print`)

```rust
fn main() {
    let x = println!("Hello");
    println!("{x:?}");
}
```

![](../images/ScrollTime.png)

You may not have expected:

```
Hello
()
```

`()` is the *unit type*. It's a type that denotes that you don't have a type - basically the same as `void` in C languages. So *everything* in Rust is typed, even functions that return nothing actually return `()`.

This is a bit of code golf - but there are times it'll be useful to know!