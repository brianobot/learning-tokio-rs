use std::panic;

use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};



#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let _handle = tokio::spawn(async move {
            // the idea is that when the tokio runtime started it already created a worker thread
            // and actions like this basically push those tasks onto 
            // this is a tokio task
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    let mut db = HashMap::<String, Vec<u8>>::new();
    
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Get(get) => {
                if let Some(value) = db.get(get.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            },
            Set(set) => {
                db.insert(set.key().to_string(), set.value().to_vec());
                Frame::Simple("OK".to_string())
                
            },
            cmd => panic!("unimplemented"),
        };
        
        connection.write_frame(&response).await.unwrap();
    }
}