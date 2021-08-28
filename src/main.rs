use std::io::prelude::*;
use std::net::{TcpListener, SocketAddr, TcpStream};

fn main() {
    let port = 8080;

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port))).unwrap();

    eprintln!("Starting application on port {}", port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handler(stream)
    }
}

fn handler(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let body = String::from("Hello world");
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);

    eprintln!("Response: {}", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
