# Aside: Unit Tests

Cargo has also created a `lib.rs` file instead of a `main.rs` file. As we said earlier, this makes the package a *library* --- you can't run it, it's designed to be used from other applications. The content of `main.rs` is *very* different:

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

Cargo has created:
* A single function named `add` that simply adds two parameters and returns the result.
* An entire unit test system, that asserts that `2+2` still equals `4`.

## Understanding the Tests

The mysterious line:

```rust
#[cfg(test)]
```

Is a *compiler directive*. It tells Cargo (and `rustc` underneath) to *only* compile the next section if you are compiling in the "test" configuration. Your unit tests won't take up *any* extra space in a deployed version of your library.

```rust
mod tests {
    use super::*;
    ..
}
```

`mod tests` is declaring a *module*. Modules can also be declared as files and directories (you'll see that later). They serve as a namespace (so everything inside is `tests::name`), a scope (variables inside a module aren't visible from outside without `pub`, and won't pollute the namespaces of other modules), and often a compilation unit - modules can often be compiled in parallel.

You draw elements from other modules into the current one with `use`. `use super::*` means "use everything from the parent module."

Finally, the test itself:

```rust
#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
```

You can annotate functions, structures, enums and lots of other things in Rust with `#[..]` annotations. These apply to whatever is defined next. In this case, marking a function as a `test` adds it to Cargo's unit-test runner.

`add` only works because we imported everything from `super`. You could also write `super::add(2, 2);`.

`assert_eq!` is an assertion macro that panics your program if the two arguments are not equal.

## Run the Tests

So that's a very long-winded check that 2+2 equals 4. Let's run it. You can run unit tests for the current workspace entry at any time by typing:

```
cargo test
```

You'll see something like this:

```
   Compiling hello_auth v0.1.0 (C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\src\hello_auth)
    Finished test [unoptimized + debuginfo] target(s) in 0.31s
     Running unittests src\lib.rs (C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\target\debug\deps\hello_auth-9e01bca15e1e38fb.exe)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests hello_auth

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

**Good news: 2+2 does equal 4.**

You can also test your whole workspace at once by typing:

```
cargo test --all
```