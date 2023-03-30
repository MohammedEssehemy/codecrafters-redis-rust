use anyhow::Result;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

mod command;
mod db;
mod resp;

use command::Command;
use db::DB;
use resp::Connection as RespConnection;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let db = Arc::new(Mutex::new(DB::new()));
    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((stream, _)) => {
                println!("accepted new connection");
                let db = Arc::clone(&db);
                tokio::spawn(async move {
                    handle_connection(stream, db).await.unwrap_or_else(|e| {
                        eprintln!("error while handling connection {e}");
                    });
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_connection(stream: TcpStream, db: Arc<Mutex<DB>>) -> Result<()> {
    let mut conn = RespConnection::new(stream);
    while let Some((cmd, args)) = conn.read_command().await? {
        let response = Command::reply(&cmd, args, db.clone());
        conn.write_value(&response).await?;
    }

    Ok(())
}
