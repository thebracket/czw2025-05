# Mutable Borrowing

So what if you want to have the function change things?

You of course declare `s` as `mut`able. But that's not enough! You also have to *explicitly* mark the borrow as mutable and the lend as mutable!

```rust
fn print_it(s: &mut String) { // &mut borrow
    println!("{s}");
    *s += "2"; // Notice the "*"
}

fn main() {
    let mut s = "my string".to_string(); // Must be mutable
    print_it(&mut s); // Must explicitly make the lend mutable
    println!("{s}");
}
```

When you do an `&mut` you aren't saying "here, borrow my car" - you are saying "here, borrow my car and if you drive it over a cliff, that's ok". You are giving the function permission to do whatever it wants. That's why it's marked explicitly. So many bugs occurred in older programs because of unexpected mutation.

So what's up with the `*`. Star (dereference) is the opposite of `&` (reference). The function sees the `String` as an `&mut String`. Making changes directly would be trying to change the internal type `MutRef<String>` - so you have to dereference to get to the inner string. There's syntax sugar to help with this for complex types and members - but it's not automatic for a bare type like this.