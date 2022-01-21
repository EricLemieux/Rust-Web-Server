use crate::http::http_status::HttpStatus;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::{EnumString, ToString};

pub mod http_status;

#[derive(Debug, PartialEq, EnumString, ToString)]
pub enum HttpMethod {
    GET,
    PUT,
    POST,
    HEAD,
    OPTIONS,
}

#[derive(Debug, PartialEq, EnumString, ToString)]
pub enum HttpVersion {
    #[strum(serialize = "HTTP/1.1")]
    Http1_1,
}

#[derive(Debug)]
pub struct HttpRequest {
    version: HttpVersion,
    method: HttpMethod,
    route: String,
    body: String,
    headers: HashMap<String, String>,
}

impl FromStr for HttpRequest {
    type Err = String;

    /// Construct a HttpMessage from a string message received.
    ///
    /// Parsing based on the structure defined in this article.
    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the message into two based on the blank line that separates the headers from the
        // message body.
        let split_request: Vec<&str> = s.split("\r\n\r\n").collect();
        let raw_headers: &str = split_request.get(0).unwrap();
        let body: &str = split_request.get(1).unwrap();

        let raw_header_components: Vec<&str> = raw_headers.split("\r\n").collect();
        let start_line: &str = raw_header_components.get(0).unwrap();
        let first_line_components: Vec<_> = start_line.split(' ').collect();

        let mut headers: HashMap<String, String> = HashMap::new();

        for element in raw_header_components.split_at(1).1.iter() {
            // TODO: This can be upgraded to be more robust with a regular expression.
            let header: Vec<&str> = element.split(": ").collect();

            let key: &str = header.get(0).unwrap();
            let value: &str = header.get(1).unwrap();

            headers.insert(String::from(key), String::from(value));
        }

        let method = HttpMethod::from_str(first_line_components.get(0).unwrap()).unwrap();
        let route: &str = first_line_components.get(1).unwrap();
        let version_component: &str = first_line_components.get(2).unwrap();
        let version: HttpVersion = match HttpVersion::from_str(version_component) {
            Ok(version) => version,
            Err(error) => {
                return Err(format!(
                    "Unable to parse the http version due to error: {}",
                    error
                ))
            }
        };

        Ok(HttpRequest {
            version,
            method,
            route: String::from(route),
            body: String::from(body),
            headers,
        })
    }
}

impl ToString for HttpRequest {
    fn to_string(&self) -> String {
        let first_line = format!(
            "{} {} {}",
            self.method.to_string(),
            self.route,
            self.version.to_string()
        );
        let headers = "Foo: Bar";

        format!("{}\r\n{}\r\n\r\n{}", first_line, headers, self.body)
    }
}

#[derive(Debug)]
struct HttpResponse {
    version: HttpVersion,
    status: HttpStatus,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpResponse {
    fn new() -> Self {
        HttpResponse {
            version: HttpVersion::Http1_1,
            status: HttpStatus::OK,
            headers: Default::default(),
            body: "".to_string(),
        }
    }
}

impl ToString for HttpResponse {
    fn to_string(&self) -> String {
        format!(
            "{} {}\r\n\r\n{}",
            self.version.to_string(),
            self.status.to_string(),
            self.body
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_to_string() {
        let request = HttpRequest {
            version: HttpVersion::Http1_1,
            method: HttpMethod::GET,
            route: "/".to_string(),
            body: "".to_string(),
            headers: Default::default(),
        };

        let expected = "GET / HTTP/1.1\r\nFoo: Bar\r\n\r\n";

        assert_eq!(expected, request.to_string())
    }

    #[test]
    fn response_to_string() {
        let response = HttpResponse {
            version: HttpVersion::Http1_1,
            status: HttpStatus::OK,
            headers: Default::default(),
            body: "Hello world!".to_string(),
        };

        let expected = "HTTP/1.1 200 OK\r\n\r\nHello world!";

        assert_eq!(expected, response.to_string());
    }
}
