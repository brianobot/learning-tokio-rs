use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant}
use futures::task;


fn main() {
    let mut mini_tokio - MiniTokio::new();

    
}


struct MiniTokio {
    tasks: VecDeque<Task>
}

type Task = Pin<Box<dyn Future<>>>