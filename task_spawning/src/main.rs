use async_std::task;
use std::time::Duration;

async fn perform_task() {
    println!("Task Started");
    task::sleep(Duration::from_secs(2)).await;
    println!("Task Completed");
}

fn main() {
    task::block_on(async {
        task::spawn(perform_task());
        println!("task spawned");
        task::sleep(Duration::from_secs(3)).await;
    });
}
