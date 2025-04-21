# Understanding Hello World

Your new project will have automatically created a hello world program:

```rust
fn main() {
    println!("Hello, World.");
}
```

## Functions

The line `fn main` creates a function. You need an entry point in executables (it can be overridden but is usually `main`). It's similar to:

```python3
if __name__ == "__main__":
    print("Hello, World.")
```

In Python, the script starts running and you check to see if you are starting as a main function or module initializer---in Rust, `main` starts.

## Scope

The function is enclosed in `{ ... }`. That's a *scope* - the *main scope*. We'll talk a lot about scopes later.

## Why is `println` so suprising?

> I sometimes quip that `Hello World` is the *worst* way to start with Rust, because you immediately run into something unusual!

Macros in Rust *always* end in `!`. We'll talk a lot about macros, but for now: a macro is a way to extend the standard Rust syntax and define your own commands. `println` works differently to most functions: it can have a variable number of parameters, complex format strings, and accepts a wide range of types. It's actually compiler assisted. It's better to have:

|Rust|Python|
|----|------|
|`println!("Hello, World");`|`print("Hello, World.")`|

As opposed to what's really happening with both:

1. Parse the string and apply formatting.
2. Obtain access to `stdout` (Rust obtains a lock for safe theading).
3. Call the operating system syscall that prints to the screen.
4. Release the lock.

## Semicolons

Notice that statement lines end with `;`. Most lines in Rust do; we'll talk about this later.
