# Quick workshop: 

Build a program that calculates the area of a circle from a radius and prints the result to 2 decimal places. 

* Set PI to 3.142.
* Use the formula PI * (radius * radius)

![](../images/ScrollTime.png)

-------
Answer:


```rust
fn main() {
    let pi = 3.142;
    let radius = 10.0;
    let area = pi * (radius * radius);
    println!("{area:.2}");
}
```

> You can get a more useful PI constant with `use std::f32::consts::PI`.