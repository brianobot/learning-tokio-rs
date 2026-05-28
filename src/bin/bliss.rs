use std::{thread, time::{Duration, Instant}};



#[tokio::main]
async fn main() {
    let now = Instant::now();

    let a = tokio::spawn({
        perform_some_action()
    });

    let b = tokio::spawn({
        perform_some_action()
    });

    let c = tokio::spawn({
        perform_some_action()
    });

    let d = tokio::spawn({
        perform_some_action()
    });
    
    println!("Elapsed Time: {:?}s", now.elapsed().as_secs());
}


async fn perform_some_action() {
    thread::sleep(Duration::from_secs(2));
}