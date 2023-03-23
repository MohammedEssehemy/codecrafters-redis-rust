// Uncomment this block to pass the first stage
use std::{net::TcpListener, io::Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                stream.write_fmt(format_args!("+PONG\r\n")).unwrap();
                // stream.
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
