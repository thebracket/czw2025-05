fn main() {
    let x = 3;
    let y = "Herbert";
    let z = 3.2;
    println!("Hello {x} {y} {z}");
    
    let x: i32 = 4;
    let y: i8 = x as i8;
    let master = x + z as i32;
    
    let x:i32 = 4;
    let y:i8 = x.try_into().unwrap();
    
    // Next part
    let x = 3;
    let x = x + 1;
}
