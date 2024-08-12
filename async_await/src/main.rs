use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let result = my_async_function().await;
    println!("The result is {}", result);

    //Chaining
    let first = first_async().await;
    let second = second_async(first).await;
    println!("Final Result: {}", second);

    //Error Handling
    match might_fail(false).await {
        Ok(val) => println!("Success: {}", val),
        Err(err) => println!("Error: {}", err)
    }
    
    // Concurrent tasks
    let t1 = task_one();
    let t2 = task_two();
    tokio::join!(t1, t2);

    println!("Both Tasks completed");
}

async fn my_async_function() -> i32 {
    42
}

//Chaining Async Opereations

async fn first_async() -> i32 {
    my_async_function().await
}

async fn second_async(num: i32) -> i32 {
    num * 2
}


// Async/Await in error handling

async fn might_fail(flag: bool) -> Result<i32, &'static str> {
    if flag {
        Ok(11)
    }else {
        Err("Something in the way")
    }
}


// Building Concurrent tasks

async fn task_one() {
    sleep(Duration::from_secs(3)).await;
    println!("Task One Completed")
}

async fn task_two() {
    sleep(Duration::from_secs(6)).await;
    println!("Task two Completed")
}