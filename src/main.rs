use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};



#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let handle = tokio::spawn(async move {
            // the idea is that when the tokio runtime started it already created a worker thread
            // and actions like this basically push those tasks onto 
            // this is a tokio task
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}