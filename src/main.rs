use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};

mod resp;
use resp::{Connection as RespConnection, Value as RespValue};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((stream, _)) => {
                println!("accepted new connection");
                tokio::spawn(async move {
                    handle_connection(stream).await.unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_connection(stream: TcpStream) -> Result<()> {
    let mut conn = RespConnection::new(stream);
    while let Some((command, args)) = conn.read_command().await? {
        let response = match command.to_ascii_lowercase().as_str() {
            "ping" => RespValue::SimpleString("PONG".to_string()),
            "echo" => {
                if args.len() == 1 {
                    args.first().unwrap().clone()
                } else {
                    RespValue::Error("echo requires exactly one argument".to_string())
                }
            }
            _ => RespValue::Error(format!("command not implemented: {}", command)),
        };

        conn.write_value(&response).await?;
    }

    Ok(())
}
