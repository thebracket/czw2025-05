# Scope Expressions

Scopes can also return values:

```rust
fn main() {
    let x = {
        4*3
    };
    println!("{x}");
}
```

> Notice no `;` - that's because it is implicitly returning the value from the scope, just like a block on a Jupyter notebook.

You can even use conditional expressions:

```rust
fn main() {
    let y = 3;
    let x = if y > 2 {
        "Bigger"
    } else {
        "Smaller"
    };
    println!("{x}");
}
```

Notice that the `let` statement has a semicolon denoting the end - but the scope returns don't; the value "falls out" of the expression.

You can go pretty scope crazy:

```rust
fn main() {
    let x = {
        let mut acc = 0;
        for i in 0 .. 10 {
            acc += i;
        }
        acc
    };
    println!("{x}");
}
```