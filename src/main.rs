// Uncomment this block to pass the first stage
use std::{
    io::Read,
    io::Write,
    net::{TcpListener, TcpStream},
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 512];
    loop {
        // Wait for the client to send us a message but ignore the content for now
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            println!("client closed the connection");
            break;
        }

        stream.write("+PONG\r\n".as_bytes()).unwrap();
    }
}
