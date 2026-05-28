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
    let (tx, rx) = mpsc::channel::<Command>(32);
}