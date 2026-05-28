use std::io::Cursor;

use bytes::BytesMut;
use tokio::{io::AsyncReadExt, net::TcpStream};
use mini_redis::{Frame, Result};


struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection { stream, buffer: BytesMut::with_capacity(4096) }
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        let mut buf = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buf) {
            
        }
    }   
    
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("Connection reset by peer".into());
                }
            }
        }
    }

    pub async fn write_frame(&mut self, frame: Frame) -> Result<()> {
        
    }
}

#[tokio::main]
async fn main() {
    
}