use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use tokio::time::MissedTickBehavior::Delay;


struct Delay {
    when: Instant
}


#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_secs(2);
    let future = Delay { when };

    
}

