use std::io::prelude::*;
use std::net::{TcpListener, SocketAddr, TcpStream};
use std::fmt;
use std::str::FromStr;
use strum_macros::EnumString;

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
    let request_string = String::from_utf8_lossy(&buffer[..]);

    println!("Request: {}", request_string);

    let request_components: Vec<&str> = request_string.split("\r\n").collect();
    let first_line: &str = request_components.get(0).unwrap();
    let first_line_components: Vec<_> = first_line.split(" ").collect();

    let verb = HttpVerb::from_str(first_line_components.get(0).unwrap()).unwrap();
    let route: &str = first_line_components.get(1).unwrap();
    let version = match HttpVersion::from_str(first_line_components.get(2).unwrap()) {
        Ok(result)  => result,
        Err(error) => {
            eprintln!("Unable to parse the http version due to error: {}", error);
            stream.write("HTTP/1.1 400 OK\r\n\r\n".as_bytes());
            stream.flush().unwrap();
            return;
        },
    };

    println!("Parsed verb: {:?}", verb);
    println!("Parsed route: {:?}", route);
    println!("Parsed method: {:?}", version);

    let body = String::from("Hello world");
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);

    eprintln!("Response: {}", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[derive(Debug, EnumString, PartialEq)]
enum HttpVerb {
    GET,
    POST,
}


#[derive(Debug, EnumString, PartialEq)]
enum HttpVersion {
    #[strum(serialize = "HTTP/1.1")]
    Http1_1
}
