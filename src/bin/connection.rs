use bytes::BytesMut;
use tokio::net::TcpStream;
use mini_redis::{Frame, Result};


struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection { stream, buffer: BytesMut::with_capacity(4096) }
    }
    
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }
        }
    }

    pub async fn write_frame(&mut self, frame: Frame) -> Result<()> {
        
    }
}

#[tokio::main]
async fn main() {
    
}