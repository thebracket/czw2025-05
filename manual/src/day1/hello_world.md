# Workshop: Hello World

We're going to do this to a) validate your setup, and b) get your feet wet quickly with some Rust!

## Create a new Rust Project

At a terminal, navigate to where you want to store the new project. Then type:

```bash
cargo new hello_world
cd hello_world
```

Open the new directory in your editor. You'll see the following files:

```
src/            # The source code directory
src/main.rs     # The main file
Cargo.toml      # Build manifest
.gitignore      # Git exclusion file
.git/           # Git repo
```

Yes - `cargo new` makes a new git repo unless you are already in one. You can turn this off with `cargo new --vcs none` if you need to (It also supports Mercurial, Pijul, and Fossil natively).

## Cargo.toml

Every new project will have a `Cargo.toml` file created. Here's an annotated version of the default file:

```toml
[package]               # Every Cargo project needs a package section
name = "login_system"   # The project name, which will double as the target filename (e.g. login_system.exe)
version = "0.1.0"       # Version in semantic versioning. We'll deal more with versions later.
edition = "2024"        # Which edition of Rust. Usually 2021, may be older for some packages.

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# Defaults to an empty list. This is where you add dependencies, which we'll do later.

```

Why 2024? Every few years, Rust issues a new *edition*. This is the only time at which substantial syntax changes are permitted. Rust retain compatibility with older editions (and syntaxes) by specifying the edition to use when building.

## The `src` folder

Your source code largely lives in the `src` folder. Rust has created a `main.rs` file for you.

* If a project has a `main.rs`, it expects to have a function named `main()` - and builds an executable.
* If a project has a `lib.rs` file, it's a library. We'll make one in a moment.
* If a project has *both*, then it can be included in other projects as a library - or executed. This requires a bit of redundant typing, but is great for attaching unit tests, benchmarks and other *library* features to something you can also execute.

Let's take a quick look at `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

There's only a little here to note:

* Rust is *terse* - `fn` for `function`, very little space is wasted.
* `fn main()` creates a new *function* called `main`, with no parameters/arguments.
* `println!` has a mysterious `!`. That denotes it as a *macro*. You'll write macros later in this class. For now, a *macro* is anything that uses the in-built macro language to support syntax that doesn't fit inside regular Rust syntax.
* Click on `println` and open your "Command Palette". Search for "expand" and find "Expand Macro Recursively" (a Rust Analyzer feature).

![](/images/ExpandMacro.png)

Once you run this command, you can find out what `println!` actually does. It's remarkably complicated:

```rust
{
    $crate::io::_print($crate::fmt::Arguments::new_v1(&[], &[]));
}
```

We'll talk about macros more at the [end of this class](/day4/hour1/macros.md).

## Run "Hello World"

You can run the newly created program with `cargo run`.

Predictably, you'll see the output:

```
   Compiling hello_login_system v0.1.0 (C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\src\hello_login_system)
    Finished dev [unoptimized + debuginfo] target(s) in 1.57s
     Running `C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\target\debug\hello_login_system.exe`   
Hello, world!
```

Since we're here, it's a good time to note that this runs everything in *debug* mode. Optimizations are disabled, extra debug information is generated, and extra run-time error checks are running.

You can always skip these checks and run in optimized mode with:

```
cargo run --release
```

(Which gives the same output with a slower compilation and un-noticeably faster execution for such a tiny program)

## The `Target` Directory

Now that you've compiled/run the program, a new directory has appeared: `target/Debug`. All of your build artefacts go here. `cargo clean` will remove them for you.

## How This Compares with Python

In Python, you'd start a new project by (optionally) creating a virtual environment, making your `.py` file and running it---or maybe using the Python REPL. Python is definitely easier, but there are a lot of similarities:

`Cargo` acts as as the build tool, which Python doesn't need because Python is interpreted. `Cargo` also fills in the role of `pip` (and similar tools) handling your dependencies.