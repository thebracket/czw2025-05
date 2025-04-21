# If Statements

If statements are pretty fundamental, and a lot like other languages.

This should look familiar:

```python3
x=3
y=4
if x > 4:
    print("Bigger")
else:
    print("Smaller")
```

The Rust is pretty much the same, but with explicit scopes (brackets, not whitespace):

```rust
fn main() {
    let x = 3;
    let y = 4;
    if x > y {
        println!("Bigger");
    } else {
        println!("Smaller");
    }
}
```

Boolean operators use C-style short-hand rather than long `and`, `or`, etc. Operators:

|English|Python|Rust|
|-------|------|----|
|And|and|&&|
|Or|or|\|\||
|Not|not|!|

For example:

```rust
fn main() {
    let x = 3;
    let y = 4;
    if x > y && x >= y {
        println!("Bigger");
    } else {
        println!("Smaller");
    }
}
```

There's no special `elif`:

```rust
fn main() {
    let x = 3;
    let y = 4;
    if x > y {
        // Do something
    } else if x < y {
        // Do something else
    } else if x == 4 {
        // Do something other (notice `==` just like Python)
    } else {
        // Etc
    }
}
```