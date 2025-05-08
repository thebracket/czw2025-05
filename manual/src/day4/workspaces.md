# Workspaces

We mentioned workspaces on day 1, and the `code` directory in the project IS a workspace. But we aren't really taking advantage of them.

Create a new project:

```bash
cargo new my_workspace
cd my_workspace
```

Edit `Cargo.toml` to include:

```toml
[workspace]
members = []
```

Now add a sub-project:

```bash
cargo new hello_workspace
```

Take a look at `Cargo.toml`. and your project has been added to the workspace.

## Shared Dependencies

You can edit the *top* `Cargo.toml` to include a section called `[workspace.dependencies]`:

```toml
[workspace.dependencies]
serde = { version = "1.0.219", features = ["derive"] }
```

In your child project, you can now add a dependency:

```toml
[dependencies]
serde.workspace = true
```

Now all of your crates will share the same version, build setup and feature flags.