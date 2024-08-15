use async_std::task;
use futures::join;
use std::time::Duration;
use tokio::time::{sleep};

// A simple async function that simulates some work

async fn async_task(name: &str, duration: u64) {
    println!("Task {} started", name);
    task::sleep(Duration::from_secs(duration)).await;
    println!("Task {} completed", name);
}

#[tokio::main]
async fn main() {
    task::block_on(async {
        let task1 = async_task("one", 2);
        let task2 = async_task("Two", 3);

        //Run Both Tasks COncurrently
        join!(task1, task2);
    });

    //Tokio Executor
    let handle1 = tokio::spawn(async { task_one().await });
    let handle2 = tokio::spawn(async { task_two().await });

    // Wait for both tasks to complete
    let _ = tokio::join!(handle1, handle2);
    println!("Both tasks completed");
}


//Using Tokio Executor
async fn task_one() {
    sleep(Duration::from_secs(2)).await;
    println!("Task one completed");
}

async fn task_two() {
    sleep(Duration::from_secs(3)).await;
    println!("Task two completed");
}