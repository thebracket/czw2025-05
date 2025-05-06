fn hello_thread(n: u32) {
    println!("Hello from thread {n}");
}

fn main() {
    println!("Hello from the main thread");

    let mut handles = Vec::new();
    for i in 0..10 {
        let thread_handle = std::thread::spawn(
            move || { hello_thread(i) }
        );
        handles.push(thread_handle);
    }
    //thread_handle.join().unwrap();
    handles.into_iter().for_each(|h| h.join().unwrap());
}
