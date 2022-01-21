use std::borrow::Borrow;
use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::{EnumString, ToString};

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

struct HttpStatus {
    code: u16,
    reason_phrase: &'static str,
}

impl HttpStatus {
    // pub const OK: HttpStatus = HttpStatus::new(200, "Ok");
    pub const OK: HttpStatus = HttpStatus {
        code: 200,
        reason_phrase: "Ok",
    };

    // const AVAILABLE: Vec<&'static HttpStatus> = vec![];
    const AVAILABLE: [&'static HttpStatus; 1] = [&HttpStatus::OK];

    pub fn from(code: u16) -> Result<&'static Self, String> {
        for i in HttpStatus::AVAILABLE {
            if i.code.eq(&code) {
                return Result::Ok(i.borrow());
            }
        }

        return Result::Err(String::from("No matching http status code found"));
    }

    // fn new(code: u16, message: &str) -> Self {
    //     HttpStatus {
    //         code,
    //         reason_phrase: message,
    //     }
    // }
}

// /// Http status codes
// /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
// #[derive(Debug, PartialEq, EnumString, ToString)]
// pub enum HttpStatus {
//     // 1xx
//     Continue,
//     SwitchingProtocols,
//     Processing,
//     EarlyHints,
//
//     // 2xx
//     OK,
//     Created,
//     Accepted,
//     NonAuthoritativeInformation,
//     NoContent,
//     ResetContent,
//     PartialContent,
//     MultiStatus,
//     AlreadyReported,
//     ImUsed,
//
//     // 3xx
//     MultipleChoices,
//     MovedPermanently,
//     MovedTemporarily,
//     SeeOther,
//     NotModified,
//     UseProxy,
//     TemporaryRedirect,
//     PermanentRedirect,
//
//     // 4xx
//     BadRequest,
//     Unauthorized,
//     PaymentRequired,
//     Forbidden,
//     NotFound,
//     MethodNotAllowed,
//     NotAcceptable,
//     ProxyAuthenticationRequired,
//     RequestTimeout,
//     Conflict,
//     Gone,
//     LengthRequired,
//     PreconditionFailed,
//     PayloadTooLarge,
//     UriTooLong,
//     UnsupportedMediaType,
//     RangeNotSatisfiable,
//     ExpectationFailed,
//     IAmATeaPot,
//     MisdirectedStatus,
//     UnprocessableEntity,
//     Locked,
//     FailedDependency,
//     TooEarly,
//     UpgradeRequired,
//     PreconditionRequired,
//     TooManyRequests,
//     RequestHeaderFieldsTooLarge,
//     UnavailableForLegalReasons,
//
//     // 5xx
//     InternalServerError,
//     NotImplemented,
//     BadGateway,
//     ServiceUnavailable,
//     GatewayTimeout,
//     HttpVersionNotSupported,
//     VariantAlsoNegotiates,
//     InsufficientStorage,
//     LoopDetected,
//     NotExtended,
//     NetworkAuthenticationRequired,
// }
//
// impl HttpStatus {
//     pub fn get_code(&self) -> int32 {
//         match self {
//             // 1xx
//             HttpStatus::Continue => 100,
//             HttpStatus::SwitchingProtocols => 101,
//             HttpStatus::Processing => 102,
//             HttpStatus::EarlyHints => 103,
//
//             // 2xx
//             HttpStatus::OK => 200,
//             HttpStatus::Created => 201,
//             HttpStatus::Accepted => 202,
//             HttpStatus::NonAuthoritativeInformation => 203,
//             HttpStatus::NoContent => 204,
//             HttpStatus::ResetContent => 205,
//             HttpStatus::PartialContent => 206,
//             HttpStatus::MultiStatus => 207,
//             HttpStatus::AlreadyReported => 208,
//             HttpStatus::ImUsed => 226,
//
//             // 3xx
//             HttpStatus::MultipleChoices => 300,
//             HttpStatus::MovedPermanently => 301,
//             HttpStatus::MovedTemporarily => 302,
//             HttpStatus::SeeOther => 303,
//             HttpStatus::NotModified => 304,
//             HttpStatus::UseProxy => 305,
//             HttpStatus::TemporaryRedirect => 307,
//             HttpStatus::PermanentRedirect => 308,
//
//             // 4xx
//             HttpStatus::BadRequest => 400,
//             HttpStatus::Unauthorized => 401,
//             HttpStatus::PaymentRequired => 402,
//             HttpStatus::Forbidden => 403,
//             HttpStatus::NotFound => 404,
//             HttpStatus::MethodNotAllowed => 405,
//             HttpStatus::NotAcceptable => 406,
//             HttpStatus::ProxyAuthenticationRequired => 407,
//             HttpStatus::RequestTimeout => 408,
//             HttpStatus::Conflict => 409,
//             HttpStatus::Gone => 410,
//             HttpStatus::LengthRequired => 411,
//             HttpStatus::PreconditionFailed => 412,
//             HttpStatus::PayloadTooLarge => 413,
//             HttpStatus::UriTooLong => 414,
//             HttpStatus::UnsupportedMediaType => 415,
//             HttpStatus::RangeNotSatisfiable => 416,
//             HttpStatus::ExpectationFailed => 417,
//             HttpStatus::IAmATeaPot => 418,
//             HttpStatus::MisdirectedStatus => 421,
//             HttpStatus::UnprocessableEntity => 422,
//             HttpStatus::Locked => 423,
//             HttpStatus::FailedDependency => 424,
//             HttpStatus::TooEarly => 425,
//             HttpStatus::UpgradeRequired => 426,
//             HttpStatus::PreconditionRequired => 428,
//             HttpStatus::TooManyRequests => 429,
//             HttpStatus::RequestHeaderFieldsTooLarge => 431,
//             HttpStatus::UnavailableForLegalReasons => 451,
//
//             // 5xx
//             HttpStatus::InternalServerError => 500,
//             HttpStatus::NotImplemented => 501,
//             HttpStatus::BadGateway => 502,
//             HttpStatus::ServiceUnavailable => 503,
//             HttpStatus::GatewayTimeout => 504,
//             HttpStatus::HttpVersionNotSupported => 505,
//             HttpStatus::VariantAlsoNegotiates => 506,
//             HttpStatus::InsufficientStorage => 507,
//             HttpStatus::LoopDetected => 508,
//             HttpStatus::NotExtended => 510,
//             HttpStatus::NetworkAuthenticationRequired => 511,
//         }
//     }
//  }

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
    status: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpResponse {
    fn new() -> Self {
        HttpResponse {
            version: HttpVersion::Http1_1,
            status: "".to_string(),
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
            self.status,
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
            status: "200 OK".to_string(),
            headers: Default::default(),
            body: "Hello world!".to_string(),
        };

        let expected = "HTTP/1.1 200 OK\r\n\r\nHello world!";

        assert_eq!(expected, response.to_string());
    }

    // Http status tests
    // This should probably be refactored to somewhere else
    #[test]
    fn status_from_code() {
        let status = HttpStatus::from(200).unwrap();

        assert_eq!(200, status.code);
        assert_eq!("Ok", status.reason_phrase);
    }

    #[test]
    fn status_from_code_that_does_not_exist() {
        let status = HttpStatus::from(600);

        assert_eq!(true, status.is_err());
    }
}
