use std::{collections::HashMap, panic};

use bytes::Bytes;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

type DB = Arc<Mutex<HashMap<String, Bytes>>>;


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db = Arc::new(Mutex::new(HashMap::new()));
    
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        let _handle = tokio::spawn(async move {
            // the idea is that when the tokio runtime started it already created a worker thread
            // and actions like this basically push those tasks onto 
            // this is a tokio task
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: DB) {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Get(get) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(get.key()) {
                    println!("Getting Value from the db");
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            },
            Set(set) => {
                println!("Settting Value to the db");
                let mut db = db.lock().unwrap();
                db.insert(set.key().to_string(), set.value().clone());
                Frame::Simple("OK".to_string())
                
            },
            _cmd => panic!("unimplemented"),
        };
        
        connection.write_frame(&response).await.unwrap();
    }
}