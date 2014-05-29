// Based on example from https://github.com/Ogeon/rustful

extern crate rustful;
extern crate http;
use rustful::{Server, Router, Request, Response};
use http::method::Get;

fn lang_detect(request: &Request, response: &mut Response) {
    match response.write("Hello!".as_bytes()) {
        Err(e) => println!("error: {}", e),
        _ => {}
    }
}

fn main() {
    let routes = [
        (Get, "/", lang_detect)
    ];

    let server = Server {
        handlers: Router::from_routes(routes),
        port: 8080
    };

    server.run();
}
