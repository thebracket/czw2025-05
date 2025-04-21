# Cargo Workspaces

If you're working with more than one related project, `workspaces` can help you:

* Workspaces combine all of your builds into a single `target` folder. No need to find every single `target/` folder when cleaning up.
* Workspaces share downloads. If you have a bunch of projects with shared dependencies, workspaces build them once---and share the result. Faster compilation, less disk space usage.
* A lot of cargo commands can be run in the "workspace root" with `--all` as a command-line flag, and will operate on all of them. Run all of your tests with `cargo test --all`, or build everything with `cargo build --all`. Beware: `cargo run --all` really will try and run every program you've created in this workspace.

> While working on Hands-on Rust, I had so many examples outside of a workspace that I ran out of disk space and realized I was using hundreds of gigabytes with multiple copies of Legion and Bracket-Lib. Moving into a workspace meant I only had a single copy of each library, and was using a reasonable amount of disk space.

## Our Current Setup

Go back to your source folder for today. In my case `c:\users\herbert\rust\live`.

Open `Cargo.toml` and add a `workspace` section:

```toml
[workspace]
members = []
```

The workspace automatically includes the top-level `src` directory. This is the *parent*
workspace.

## Add a project

In your terminal, from the root folder of your project, type:

```bash
cargo new hello2
```

A `hello2` directory appears, `hello2` is added to your workspace members. Type:

```bash
cd hello2
cargo run
```

The program runs. Notice that a second target directory *did not appear*---workspaces share all build artefacts. This not only saves space, but makes compilation faster---workspace projects can share build artefacts.

## Mark The Root

It's common, if you have a lot of sub-projects and no real "main" project, to accidentally run the top-level project. As a good practice, I like to change the `src/main.rs` at the top to print "You probably wanted to run one of the nested workspaces."

Let's do that.