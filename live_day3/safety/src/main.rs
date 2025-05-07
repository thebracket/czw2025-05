use std::ops::Add;

struct Point {
    x: f32,
    y: f32,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let a =  Point { x: 0.0, y: 0.0 };
    let b =  Point { x: 1.0, y: 4.0 };
    let result = Point { x: a.x + b.x, y: a.y + b.y };
    let result = a + b;
    
    let mut n:u8 = 255;
    let (n, overflowed) = n.overflowing_add(1);
    let Some(n) = n.checked_add(1) else {
        // Ooops;
        panic!();
    };
    println!("{}", n);
}