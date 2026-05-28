use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};


struct Delay {
    when: Instant,
    ping: u32
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Hello World");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            self.ping += 1;
            println!("Waiting...{}", self.ping);
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_secs(2);
    let future = Delay { when, ping: 0 };

    let out = future.await;
    assert_eq!(out, "done");
}

