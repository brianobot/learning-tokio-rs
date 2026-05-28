use std::{thread, time::{Duration, Instant}};



#[tokio::main]
async fn main() {
    let now = Instant::now();
    
    perform_some_action().await;
    perform_some_action().await;
    perform_some_action().await;
    perform_some_action().await;
}


async fn perform_some_action() {
    thread::sleep(Duration::from_secs(2));
}