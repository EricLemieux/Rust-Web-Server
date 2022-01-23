use rust_web_server::http::http_status::HttpStatus;
use rust_web_server::http::HttpResponse;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;

mod http;

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

    stream.read_exact(&mut buffer).unwrap();
    let request_string: String = String::from_utf8_lossy(&buffer[..]).parse().unwrap();

    eprintln!("{}", request_string);

    let message = match rust_web_server::http::HttpRequest::from_str(request_string.as_str()) {
        Ok(message) => message,
        Err(error) => {
            eprintln!("{}", error);

            let mut res = HttpResponse::new();
            res.status(HttpStatus::BAD_REQUEST);

            stream.write_all(res.to_string().as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        }
    };

    println!("Parsed message: {:?}", message);

    let body = String::from("Hello world");
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );

    eprintln!("Response: {}", response);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
