use bytes::Bytes;

use tokio::sync::mpsc;



#[derive(Debug)]
enum Command {
    Get {
        key: String
    },
    Set {
        key: String,
        val: Bytes
    }
}

#[tokio::main]
async fn main() {
    // in this approach, we can create a task to manage central processing like a connection
    // and the forward all the messages (data) needed to be processeed to that task
    // the forwarders are the senders, the processing unit is the receiver
    // 
    // also notice that the channel capacity is set to 32, if messages are not processed as quickly as 
    // they are received, once it reaches 32, calling send().await would go to sleep until the channel is fred up
    let (tx, rx) = mpsc::channel(32);

    // sending from multiple tasks is done by cloning the sender
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("A").await.unwrap();
    });
}