# Public vs Private: Hello World Service

Let's create a "hello world" service.

### Step 1: Cleanup the Default Code

Delete the `add` function, and the unit test (keeping the framework).

Your code now looks like this:

```rust

#[cfg(test)]
mod tests {
    use super::*;
}
```

> Clippy complains that we aren't using any functions from `super`. That's ok for now.

### Step 2: Add a function

Now we'll add a function:
```rust
pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}
```

There's a few things to notice here:

* We made the function *public* with `pub`. That means we can call it from programs that use the library.
* Rust has TWO types of string!
    * `&str` is an immutable buffer of characters in memory.
        * You usually use this for literals, such as `"Herbert"`.
        * You can refer to any `String` as an `&str` by borrowing it - with `&my_string`.
    * `String` is an all-singing, all dancing buffered string designed for modification.
        * Internally, `String` is a buffer of characters with the length stored.
        * Changing a `String` updates or replaces the buffer.
    * `format!` is another macro, used under the hood by `printn!` and other Rust formatters. [It's very powerful](https://doc.rust-lang.org/std/macro.format.html) - it can do formatting and placeholders.

### Step 3: Test the New Function

Before we integrate this with our main program, let's add a unit test to ensure that it does what we think:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello Herbert", greet_user("Herbert"));
    }
}
```

And run it with `cargo test`:

```
running 1 test
test tests::test_greet_user ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```