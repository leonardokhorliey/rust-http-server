#![allow(dead_code)]
#![warn(unused_variables)]

mod server;
mod http;

use http::{Method, Request};
use server::Server;

fn main() {
    println!("Hello, world!");
    let url = String::from("127.0.0.1:8080");
    let slice = &url[10..];
    let server = Server::new(url);
}







