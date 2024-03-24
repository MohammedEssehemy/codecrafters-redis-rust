use anyhow::Result;
use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use super::value::{parse_message, Value};

/// RESP protocol spec
/// Redis serialization protocol (RESP) specification
/// https://redis.io/docs/reference/protocol-spec

pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(512),
        }
    }

    pub async fn read_command(&mut self) -> Result<Option<(String, Vec<Value>)>> {
        loop {
            let bytes_read = self.stream.read_buf(&mut self.buffer).await?;
            if bytes_read == 0 {
                return Ok(None);
            }
            // println!("buffer {:?}", self.buffer);
            let message = parse_message(self.buffer.split())?;
            // println!("buffer ressss {:?}", message);
            if let Some((value, _)) = message {
                println!("message {value:?}");
                return value.to_command().map(|cmd| Some(cmd));
            }
        }
    }

    pub async fn write_value(&mut self, value: &Value) -> Result<()> {
        self.stream.write(value.encode().as_bytes()).await?;

        Ok(())
    }
}
