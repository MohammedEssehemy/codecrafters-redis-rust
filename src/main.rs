use anyhow::Result;
use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((mut stream, _)) => {
                println!("accepted new connection");
                tokio::spawn(async move {
                    handle_connection(&mut stream).await.unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    let mut buf = BytesMut::with_capacity(512);

    loop {
        // Wait for the client to send us a message but ignore the content for now
        let bytes_read = stream.read(&mut buf).await?;
        if bytes_read == 0 {
            println!("client closed the connection");
            break;
        }

        stream.write("+PONG\r\n".as_bytes()).await?;
    }
    Ok(())
}
