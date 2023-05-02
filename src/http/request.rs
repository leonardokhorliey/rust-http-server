use super::{Method, method::MethodError};
use std::{str::{self, FromStr}, convert::TryFrom, error::Error, fmt::{Display, Formatter, Result as FmtResult, Debug}};

pub struct Request<'buf> {

    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    fn try_from(value: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(value).or(Err(ParseError::InvalidEncoding))?;

        // repeated calls with reassigning request. More like a recursion.
        let (method, request) = Self::get_next_word_from_request(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = Self::get_next_word_from_request(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = Self::get_next_word_from_request(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method = Method::from_str(method)?; // can also call method.parse(). Would fail still since MethodError is not in scope. Requires a From impl

        let mut query_string = None;
        if let Some(q_index) = path.find('?') {
            query_string = Some(&path[(q_index + 1)..]);
            path = &path[..q_index];
        }

        Ok(Self {
            path,
            query_string,
            method
        })
    }
}

impl<'buf> Request<'buf> {
    // function to handle space and line-break character based spliting of request string
    fn get_next_word_from_request(request: &str) -> Option<(&str, &str)> {

        for (index, ch) in request.chars().enumerate() {
            if ch == ' ' || ch == '\r' {
                return Some((&request[..index], &request[(index + 1)..]))
            }
        }

        None
    }
}



pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<MethodError> for ParseError {
    
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl ParseError {

    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request Received",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Method",
            Self::InvalidMethod => "Invalid Method Received"
        }
    }
}

// must implement Display and Debug traits to implement Error trait
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}