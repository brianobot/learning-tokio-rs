use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant}
use futures::task;
use tokio::time::MissedTickBehavior::Delay;


fn main() {
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when =  Delay { when, ping: 0 }
    });

    
}


struct MiniTokio {
    tasks: VecDeque<Task>
}

type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

impl MiniTokio {
    fn new() -> Self {
        Self {
            tasks: VecDeque::new()
        }
    }

    fn spawn<F>(&mut self, future: F)
    where 
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(future));
    }

    fn run(&mut self) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }
    }
}

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
