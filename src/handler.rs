use crate::{server::Handler, http::{StatusCode, Request, Response}};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {

    fn handle_request(&self, request: &Request) -> Response {
        println!("request data: {:?}", request);
        Response::new(StatusCode::Ok, Some("Take your result".to_string()))
    }
}