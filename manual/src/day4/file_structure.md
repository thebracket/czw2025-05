# Structuring Rust Modules

You probably don't want to put everything into one file (you can - and I've debugged 20k lines of Rust in a single file. It's not always ideal!)

You've seen in tests that the `mod` keyword can make modules in your file:

```rust
mod my_module {
    fn print_it() {
        println!("Hello World");
    }
}

fn main() {
    // Access via namespace
    my_module::print_it();

    // OR import with a `use`
    use my_module::print_it;
    print_it();
}
```

That gets us part way to using multiple files.

Now create a file `my_module.rs` in the `src` directory.

`my_module`:
```rust
pub fn print_it() {
    println!("Hello");
}
```

`main`:
```rust
mod my_module;
use my_module::print_it;

fn main() {
    print_it();
}
```

You can even go wild with adding sub-modules! Create a directory `my_module` (in the same directory as `my_module.rs`). In there put a file named `mod2.rs` with a single function.

Now you can annotate `my_module.rs` to import it:

```rust
mod mod2;

pub use mod2::my_function;
```

## What's Up with the Pub?

Modules have their own privacy. You can reach into them if they are in the same file, or beneath you in the tree - but when you go sideways or back up, you can only access *public* items.