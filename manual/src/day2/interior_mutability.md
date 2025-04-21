# Sync+Send/Interior Mutability

There's a golden rule:

> You can have as many *read only/immutable* borrows as you want, exclusive or *one* mutable borrow. If something has to be changed from multiple borrows, it *must* be protected with a `Mutex` (or other synchronization).

Rust enforces safety by automatically marking types with `Sync` and `Send`. Almost everything is `Send` - it means it can be safely transmitted between threads (there are a few rare exceptions like `Rc` - single-threaded reference counting). Most things are *not* `Sync`.

A `Mutex` or an `AtomicX` implements `Sync` and protects its contents.

If everything in a structure implements `Sync`, then your structure automatically implements it too. So you can have:

```rust
struct MyStruct {
    a: Mutex<..>,
    b: Mutex<..>,
    c: AtomicU32,
}
```

Everything in that structure is `Sync` and `Send` - so the structure is, too. This allows you to use `impl` to have the structure handle locking, reducing the error-prone nature of acquiring locks from outside.