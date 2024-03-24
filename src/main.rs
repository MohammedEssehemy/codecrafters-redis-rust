use anyhow::Result;
use std::{
    env::args,
    sync::{Arc, Mutex},
};
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

    let port = match args().nth(1).as_ref().map(|s| s.as_str()) {
        Some("--port") => args().nth(2).expect("port is required"),
        _ => "6379".to_string(),
    };

    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;
    println!("Listening on port {port}");
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
        println!("cmd: {cmd}, args: {args:?}");
        let response = Command::reply(&cmd, args, db.clone());
        println!("response: {response:?}");
        conn.write_value(&response).await?;
    }

    Ok(())
}
