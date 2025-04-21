# Add a Variable and Print It

In Python, you're used to doing this:

```
>>> x=3
>>> print("Hello {}!".format(x))
Hello 3
```

It's not that different in Rust (illustrating two valid formats):

```rust
fn main() {
    let x = 3;
    println!("Hello {x}!");
    println!("Hello {}!", x);
}
```

Just like Python, there are various attributes you can set to apply formatting. For example, you can left-align the column with 10 characters:

```rust
fn main() {
    let x = 3;
    println!("Hello {x:<10}!");
}
```

Or:

```rust
fn main() {
    let x = 3.14159;
    println!("Hello {x:.2}!");
}
```

> See [https://doc.rust-lang.org/std/fmt/](https://doc.rust-lang.org/std/fmt/) for a full description of formatters.
