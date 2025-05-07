struct Cat(String);

struct CatFeeder<'a> {
    cat: &'a mut Cat
}

impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} (purring)", self.0);
    }
}

impl<'a> CatFeeder<'a> {
    fn feed(&mut self) {
        self.cat.feed();
    }
}

fn main() {
    let mut feeders = Vec::new();
    {
        let mut cats = vec![
            Cat("Frodo".to_string()),
            Cat("Bilbo".to_string()),
            Cat("Pippin".to_string()),
        ];

        for cat in cats.iter_mut() {
            feeders.push(CatFeeder{ cat })
        }
    }

    feeders.iter_mut().for_each(|f| f.feed());
}

fn borrow_me(s: &MyStruct) {
    // Do something cool here
}

struct MyStruct {
    n: i32
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct");
    }
}

impl MyStruct {
    fn new() -> Self {
        Self {n: 42}
    }

    fn get(&mut self) -> i32 {
        self.n
    }
}


fn safe_verbose(list: &Vec<i32>) {
    if let Some(n) = list.get(5) {
        // Use the element
        println!("The value of element is {}", n);
    } else {
        // Handle not available
        println!("There is no value at index 5");
    }
}

fn less_verbose(list: &Vec<i32>) {
    let Some(n) = list.get(5) else {
        return
    };
    println!("The value of element is {}", n);
}

fn safe_panic(list: &Vec<i32>) {
    println!("{}", list[5]);
}

fn not_good(list: &Vec<i32>) {
    unsafe {
        println!("{}", list.get_unchecked(5));
    }
}