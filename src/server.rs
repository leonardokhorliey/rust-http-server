use std::{net::TcpListener, io::Read};
use crate::Request;

pub struct Server {
    address: String,

}

impl Server {

    pub fn new(_address: String) -> Server {
        Server {
            address: _address
        }
    }

    pub fn run(&self) {
        println!("Server listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer =[0; 1024]; // initialize zero-value array with length 1024
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer as &[u8]) {
                                Ok(request) => {

                                },
                                _ => {}
                            };
                        },
                        Err(e) => println!("Bad request: {}", e)
                    };
                },
                _ => {}
            };
        }
    }



}