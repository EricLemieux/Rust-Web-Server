use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
pub enum HttpMethod {
    GET,
    PUT,
    POST,
    HEAD,
    OPTIONS,
}

#[derive(Debug, PartialEq, EnumString)]
pub enum HttpVersion {
    #[strum(serialize = "HTTP/1.1")]
    Http1_1,
}

#[derive(Debug)]
pub struct HttpMessage {
    version: HttpVersion,
    method: HttpMethod,
    route: String,
    body: String,
    headers: HashMap<String, String>,
}

impl HttpMessage {
    /// Construct a HttpMessage from a string message received.
    ///
    /// Parsing based on the structure defined in this article.
    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages
    pub fn new(request: String) -> Result<HttpMessage, String> {
        // Split the message into two based on the blank line that separates the headers from the
        // message body.
        let split_request: Vec<&str> = request.split("\r\n\r\n").collect();
        let raw_headers: &str = split_request.get(0).unwrap();
        let body: &str = split_request.get(1).unwrap();

        let raw_header_components: Vec<&str> = raw_headers.split("\r\n").collect();
        let start_line: &str = raw_header_components.get(0).unwrap();
        let first_line_components: Vec<_> = start_line.split(" ").collect();

        let mut headers: HashMap<String, String> = HashMap::new();

        for element in raw_header_components.split_at(1).1.iter() {
            // TODO: This can be upgraded to be more robust with a regular expression.
            let header: Vec<&str> = element.split(": ").collect();

            eprintln!("Split header: {:?}", header);

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

        Ok(HttpMessage {
            version,
            method,
            route: String::from(route),
            body: String::from(body),
            headers,
        })
    }
}
