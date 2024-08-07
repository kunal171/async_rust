use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::{sleep, Duration};

//A Custom Future

struct MyFuture{
    remaining_time: u64,
}

impl Future for MyFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remaining_time == 0 {
            Poll::Ready("Future is ready!".to_string())
        }else {
            //Decrease the remaining time and return Pending
            // In a real-world scenario, this would be where we check some async condition
            self.get_mut().remaining_time -= 1;
            _cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// Function that returns our custom Future
fn my_future() -> MyFuture {
    MyFuture { remaining_time: 111111111 }
}

#[tokio::main]
async fn main() {
    println!("starting the future...");

    //Using the custom Future with async/await
    let result = my_future().await;

    println!("Future completed with result: {}", result);
}
