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
use db::{NodeAddress, NodeRole, DB};
use resp::Connection as RespConnection;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Logs from your program will appear here!");
    let port = match args().position(|s| s == "--port") {
        Some(i) => args().nth(i + 1).expect("port is required"),
        _ => "6379".to_string(),
    };

    let role = match args().position(|s| s == "--replicaof") {
        Some(i) => NodeRole::Slave(NodeAddress {
            host: args().nth(i + 1).expect("master host is required"),
            port: args()
                .nth(i + 2)
                .map(|p| p.parse().expect("invalid master port"))
                .expect("master port is required"),
        }),
        _ => NodeRole::Master,
    };

    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;
    println!("Listening on port {port}");
    let db = Arc::new(Mutex::new(DB::new(role)));
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
