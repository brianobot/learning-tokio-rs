use std::{thread, time::{Duration, Instant}};



#[tokio::main]
async fn main() {
    let now = Instant::now();
    
    tokio::join!(
        perform_some_action().await,
        perform_some_action().await,
        perform_some_action().await,
        perform_some_action().await,
    );

    println!("Elapsed Time: {:?}s", now.elapsed().as_secs());
}


async fn perform_some_action() {
    thread::sleep(Duration::from_secs(2));
}