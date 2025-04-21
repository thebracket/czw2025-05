# Types

You can perform the following in Python:

```python3
x=3
y="Bert"
z=4.1
```

And just as easily in Rust (well, with a few more `let` statements!):

```rust
fn main() {
    let x = 3;
    let y = "Bert";
    let z = 4.1;
}
```

In Python, you'd expect this to work:

```python3
x=3
y="Bert"
z=4.1
print("{}".format(x+z))
```

> It answers `7.1`.

But this fails:

```python3
x=3
y="Bert"
z=4.1
print("{}".format(x+y+z))
```

> It answers "TypeError: unsupported operand type(s) for +: 'int' and 'str'".

So Python *has* types under the hood, but mostly shields you from conversions.

Now let's try this in Rust:

```rust
fn main() {
    let x = 3;
    let y = "Bert";
    let z = 4.1;
    println!("{}", x+z);
}
```

> It doesn't compile, with an error message about adding an integer to a float. The longer versions also doesn't compile.

## So What's Going On Here?

The `let` statement in Rust will try to implicitly determine the type of a variable. If it looks like an integer, it's really an `i32`. If it looks like a floating point number, it's an `f32`. UNLESS it is used in a context where it has to be a different, compatible type---in which case it could be an `i64`, an `i8`, a `u8`, etc.

Rust has *signed* (supporting positive and negative numbers) for 8, 16, 32, 64 and 128 bit types. It has unsigned (prefix `u`) for the same ranges.

You can specify the types yourself if you want to be certain:

```rust
fn main() {
    let x: i32 = 3;
    let y: f32 = 4.1;
}
```

Rust is also *very safety conscious*. You can get into all manner of hot water with type conversions; implicit type conversions have caused all manner of bugs in C and C++ code over the years. So the following won't compile:

```rust
fn main() {
    let x: i8 = 4;
    let y: i16 = 6;
    let z: i32 = 8;
    let answer = x+y+z;
}
```

## Safe Conversion with `into`

You can always safely convert *compatible* types with `.into()`. For example:

```rust
fn main() {
    let x: i8 = 4;
    let y: i32 = x.into();
}
```

This doesn't work the other way around:

```rust
fn main() {
    let x: i32 = 4;
    let y: i8 = x.into();
}
```

> Any guesses why not?

You can't safely reduce the size of a type without the risk of losing data. "4" will fit in either type, but what if x were 130? You'd lose data. That's bad.

## Giving it a Go with `try_into`

You can use `try_into` to *try* and down-convert, and throw an error if it will result in data loss:

```rust
use std::convert::TryInto;

fn main() {
    let x: i32 = 4;
    let y: i8 = x.try_into().unwrap();
}
```

> Now try it with an invalid number

## Forcing it with `as`

You can also tell Rust that you don't care about precision/data loss and to just perform a conversion:

```rust
fn main() {
    let x:i32 = 4;
    let y:i32 = 130;
    println!("{}, {}", x as i8, y as i8);
}
```

> The lesson here: prefer `into`, `try_into` if it matters, and `as` if you have to---or *know* that its safe.

## Back to the Example

Going back to:

```rust
fn main() {    
    let x = 3;
    let y = "Bert";
    let z = 4.1;
    println!("{}", x+z+y);
}
```

This won't work because you have an `i32` being added to an `f32` - and then appended to a String.

> Suggestions?

The right answer here is to make `x` an `f32` and *append* the string to the output:

```rust
fn main() {    
    let x = 3.0;
    let y = "Bert";
    let z = 4.1;
    println!("{}{y}", x+z);
}
```

> The *real* lesson here is that you have think about your types. If you know that the result is going to be a floating point number, using a floating point number for the other parts will work with a lot less jumping through hoops.