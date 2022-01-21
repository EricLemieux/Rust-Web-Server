use std::borrow::Borrow;

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

#[cfg(test)]
mod tests {
    use super::*;

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
