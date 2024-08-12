#[tokio::main]
async fn main() {
    let result = my_async_function().await;
    println!("The result is {}", result);

    //Chaining
    let first = first_async().await;
    let second = second_async(first).await;
    println!("Final Result: {}", second);
    

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