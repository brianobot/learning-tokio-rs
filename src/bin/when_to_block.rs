use std::time::Duration;



#[tokio::main]
async fn main() {
    let blocking_handle = tokio::task::spawn_blocking(|| {
        println!("Inside Spawn blocking");
        std::thread::sleep(Duration::from_secs(10));
    });

    println!("Inside the Main");
    blocking_handle.await.unwrap()
}