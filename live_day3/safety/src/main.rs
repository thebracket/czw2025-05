use std::sync::Arc;

fn main() {
    let list = vec![1, 2, 3, 4, 5];

    //safe_verbose(&list);
    //less_verbose(&list);
    //safe_panic(&list);
    //not_good(&list);

    println!("Top");
    {
        //let n = MyStruct::new();
        println!("Call");
        //borrow_me(&n);
        println!("Return");
        //let pointer = Box::new(MyStruct::new());
        let shared_pointer = Arc::new(MyStruct::new());
        let shared_pointer2 = shared_pointer.clone();
        let shared_pointer3 = shared_pointer.clone();
    }
    println!("Bottom");
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