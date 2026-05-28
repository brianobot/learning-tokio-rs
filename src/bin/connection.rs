use bytes::BytesMut;
use tokio::net::TcpStream;
use mini_redis::{Frame, Result};


struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new()
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        
    }

    pub async fn write_frame(&mut self, frame: Frame) -> Result<()> {
        
    }
}

#[tokio::main]
async fn main() {
    
}