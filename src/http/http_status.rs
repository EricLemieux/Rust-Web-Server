use std::borrow::Borrow;

/// HTTP Status codes
///
/// See: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
#[derive(Debug)]
pub struct HttpStatus {
    code: u16,
    reason_phrase: &'static str,
}

impl HttpStatus {
    // 1xx

    // 2xx
    pub const OK: HttpStatus = HttpStatus {
        code: 200,
        reason_phrase: "OK",
    };
    pub const CREATED: HttpStatus = HttpStatus {
        code: 201,
        reason_phrase: "Created",
    };
    pub const ACCEPTED: HttpStatus = HttpStatus {
        code: 202,
        reason_phrase: "Accepted",
    };

    // 3xx

    // 4xx
    pub const BAD_REQUEST: HttpStatus = HttpStatus {
        code: 400,
        reason_phrase: "Bad Request",
    };

    // 5xx

    // TODO: Figure out how to populate this with a macro.
    const AVAILABLE: [&'static HttpStatus; 4] = [
        &HttpStatus::OK,
        &HttpStatus::CREATED,
        &HttpStatus::ACCEPTED,
        &HttpStatus::BAD_REQUEST,
    ];

    pub fn from(code: u16) -> Result<&'static Self, String> {
        for i in HttpStatus::AVAILABLE {
            if i.code.eq(&code) {
                return Result::Ok(i.borrow());
            }
        }

        return Result::Err(String::from("No matching http status code found"));
    }
}

impl ToString for HttpStatus {
    fn to_string(&self) -> String {
        format!("{} {}", self.code, self.reason_phrase)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_from_code() {
        let status = HttpStatus::from(200).unwrap();

        assert_eq!(200, status.code);
        assert_eq!("OK", status.reason_phrase);
    }

    #[test]
    fn status_from_code_that_does_not_exist() {
        let status = HttpStatus::from(600);

        assert_eq!(true, status.is_err());
    }

    #[test]
    fn status_to_string() {
        let status = HttpStatus::OK;

        assert_eq!("200 OK", status.to_string())
    }
}
