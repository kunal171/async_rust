use async_std::task;
use futures::join;
use std::time::Duration;

// A simple async function that simulates some work

async fn async_task(name: &str, duration: u64) {
    println!("Task {} started", name);
    task::sleep(Duration::from_secs(duration)).await;
    println!("Task {} completed", name);
}

fn main() {
    task::block_on(async {
        let task1 = async_task("one", 2);
        let task2 = async_task("Two", 3);

        //Run Both Tasks COncurrently
        join!(task1, task2);
    });
}
