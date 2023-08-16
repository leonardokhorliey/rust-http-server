use super::status_code::StatusCode;
use std::{fmt::{Display, Formatter, Result as FmtResult, Debug}, net::TcpStream, io::{Result as IoResult, Write}};

pub struct Response {

    status_code: StatusCode,
    body: Option<String>
}

impl Response {

    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {

        Self {
            status_code,
            body
        }
    }

    // mut impl is used as static dispatch on the Write trait. For dynamic (runtime) dispatch, 'mut dyn' will be used.
    // can pass any type which implements the Write trait for the stream parameter
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {

        let body = if let Some(body) = &self.body {
            body
        } else { "" };
        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}"
        , self.status_code
        , self.status_code.reason_phrase()
        , body)

    }
}

impl Display for Response {

    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = if let Some(body) = &self.body {
            body
        } else { "" };
        write!(f, "HTTP/1.1 {} {}\r\n\r\n{}"
        , self.status_code
        , self.status_code.reason_phrase()
        , body)
    }
}