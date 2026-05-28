use std::{collections::HashMap, panic, thread, time::Duration};

use bytes::Bytes;
use mini_redis::{Connection, Frame, client};
use std::sync::{Arc, Mutex};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver},
        oneshot,
    },
    time::Sleep,
};

type DB = Arc<Mutex<HashMap<String, Bytes>>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    // let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    // let db = Arc::new(Mutex::new(HashMap::new()));
    let (tx, rx) = mpsc::channel(100);

    let manager = tokio::spawn(process_v2(rx));
    // loop {
    //     let (socket, _) = listener.accept().await.unwrap();
    //     // let db = db.clone();
    //     // let _handle = tokio::spawn(
    //     //     // process_v2(rx)
    //     //     // the idea is that when the tokio runtime started it already created a worker thread
    //     //     // and actions like this basically push those tasks onto
    //     //     // this is a tokio task
    //     //     // process_v1(socket, db).await;
    //     // );
    //     //
    // }
    //
    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        println!("About to send First Message");
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };
        tx.send(cmd).await.unwrap();
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        println!("About to send Second Message");
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx
        };
        tx2.send(cmd).await.unwrap();
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}

async fn process_v1(socket: TcpStream, db: DB) {
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
            }
            Set(set) => {
                println!("Settting Value to the db");
                let mut db = db.lock().unwrap();
                db.insert(set.key().to_string(), set.value().clone());
                Frame::Simple("OK".to_string())
            }
            _cmd => panic!("unimplemented"),
        };

        connection.write_frame(&response).await.unwrap();
    }
}

async fn process_v2(mut rx: Receiver<Command>) {
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    while let Some(cmd) = rx.recv().await {
        use Command::*;

        match cmd {
            Get { key } => {
                println!("Processing Get! key = {key}");
                client.get(&key).await.unwrap();
            }
            Set { key, val } => {
                println!("Processing Set! key = {key}, val = {val:?}");
                client.set(&key, val).await.unwrap();
            }
        }
    }

    println!("Reached the End of the Task Manager");
}
