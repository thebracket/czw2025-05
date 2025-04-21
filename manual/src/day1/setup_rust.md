# Setup Your Rust Development Environment

> Hopefully, you've already done some of this.

## Rust Setup with RustUp

If you haven't already, you need to have Rust installed on your computer.
Visit [RustUp.rs](https://rustup.rs/) and install from there. Instructions
will vary by platform.

![](/images/RustUp.png)

### Things to Know About RustUp

There are some caveats about RustUp that you need to know:

* RustUp installs Rust *for your user account*. It doesn't perform a shared installation.
* You *can* get a working environment through package managers such as `snap`, `apt`, `homebrew` and similar. It's better to use the native Rust setup, you get much faster access to updates.
* On Windows, if you don't already have it installed you will be prompted to also install the Visual C++ build tools. 
* On a Mac, you will be prompted to install the OS X development tools if you haven't already.
* On Linux, make sure you have LLVM installed.

### Verify that you have a working RustUp

> This will occur in a live demo.

In a terminal, enter the command `rustup --version`. You should see something like this:

```
rustup 1.28.1 (f9edccde0 2025-03-05)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.85.1 (4eb161250 2025-03-15)`
```

> *The version numbers will change.*

## TASK!

Make sure that `rustup` is working on your system. Without a properly installed Rust, you won't be able to participate in the live sessions.

## Updating

From time to time, you'll want to make sure that you have the latest RustUp installed. The command:

```bash
rustup self update
```

Updates the RustUp system itself. You can then update Rust to the latest version with:

```bash
rust update
```

## Adding Toolchains

You may want to target platforms other than the one you are using for development. Rust includes cross-compilation.

For example, you can support WASM by running:

```
rustup target add wasm32-unknown-unknown
```

## Clippy (Linter)

Older Rust installations didn't include clippy by default. Type `cargo clippy` and if you see an error indicating that Clippy is missing, type the following:

```bash
rustup component add clippy
```
