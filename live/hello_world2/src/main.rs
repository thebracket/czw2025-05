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

    let x = 3;
    {
        let x = 4;
        println!("{x}");
    }
    println!("{x}");

    // Control Flow
    let x = 3;
    let y = 4;
    if x == 3 && y == 4 || x > 4 && !x<2 {}
    if x > y {
        // Do something
    } else if x > 3 {
        // Another
    } else {
        // Do something else
    }

    // For loops
    let range1 = 3.0 .. 4.0;
    let range = x .. x+3;
    for i in 0..20 {
        println!("{i}");
    }

    // Loop
    let mut n = 0;
    println!("Looping 10 times");
    loop {
        n += 1;
        if n > 10 {
            break;
        }
    }
    println!("Escaped");

    // While loop
    let mut count = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }
    println!("Loop finished.");

    let x = {
        4 * 3
    };

    let answer;
    if x > 2 && y > 3 {
        answer = x;
    } else {
        answer = y;
    }
    println!("The answer is {}", answer);

    let answer = if y > 2 && x > 3 {
        12
    } else {
        13
    };

    //let x = println!("Hello");

    let mut s = "Hello".to_string();
    print_it(&mut s);
    println!("{s}");

    let n = 12;
    print_int(n);
    println!("{n}");

    let n: i128 = 32;

    let input = read_line();
    println!("You typed: {input}");
}

fn print_it(s: &mut String) {
    *s += "2";
    println!("{s}");
}

fn print_int(s: i32) {
    println!("{s}");
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}