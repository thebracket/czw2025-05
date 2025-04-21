# Borrowing

A more Rusty way to do it is to *borrow* the value:

```rust
fn print_it(s: &String) {
    println!("{s}");
}

fn main() {
    let s = "my string".to_string();
    print_it(&s);
    println!("{s}");
}
```

Adding the `&` to the argument indicates a *borrow*. You also have to add it at the call site to indicate that you want to *lend* the variable. The function call is now sending a pointer to the string, and operating on the original string.

> The "Borrow Checker" exists to stop you from falling into pitfalls doing this. C++ has had a terrible time with things like "borrow a variable that's already been deleted". We'll worry about that in more advanced sections.
