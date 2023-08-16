use std::{net::TcpListener, io::{Read, Write}};
use crate::{Request, http::{Response, StatusCode, ParseError}};

pub trait Handler {

    fn handle_request(&self, request: &Request) -> Response;
    fn handle_bad_request(&self) -> Response {
        Response::new(StatusCode::BadRequest, Some("Could not parse request".to_string()))
    }
}

pub struct Server {
    address: String,

}

impl Server {

    pub fn new(_address: String) -> Server {
        Server {
            address: _address
        }
    }

    pub fn run(&self, mut handler:impl Handler) {
        println!("Server listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer =[0; 1024]; // initialize zero-value array with length 1024
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer as &[u8]) {
                                Ok(request) => handler.handle_request(&request),
                                _ => handler.handle_bad_request()
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Connection closed before response: {}", e);
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e)
                    };
                },
                _ => {}
            };
        }
    }



}