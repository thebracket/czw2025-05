async fn hello() {
    println!("Hello, world!");
}

async fn double(n: i32) -> i32 {
    n * 2
}

use std::time::Duration;
use tokio::task::spawn_blocking;

async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    tokio::time::sleep(Duration::from_millis(time)).await;
    let result = spawn_blocking(move || {
        std::thread::sleep(Duration::from_millis(time));
    }).await;
    println!("Task {task} is done.");
}

#[tokio::main]
async fn main() {
    let result = tokio::join!(double(4), double(2));
    hello().await;
    hello_delay(10, 5).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_double() {
        
    }
}