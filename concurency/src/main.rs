#[tokio::main]
async fn main() {
    asynchronous_function().await;
}

async fn asynchronous_function() {
    println!("Hello, async world!");
}