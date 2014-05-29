// Based on example from https://github.com/Ogeon/rustful

extern crate rustful;
extern crate http;

use std::os::getenv;
use std::io::net::ip::Port;
use rustful::{Server, Router, Request, Response};
use http::method::Get;

fn lang_detect(request: &Request, response: &mut Response) {
    match response.write("Hello!".as_bytes()) {
        Err(e) => println!("error: {}", e),
        _ => {}
    }
}

/// Look up our server port number in PORT, for
/// compatibility with Heroku.
fn get_server_port() -> Port {
    getenv("PORT")
        .and_then(|s| from_str::<Port>(s.as_slice()))
        .unwrap_or(8080)
}

/// Configure and run our server.
fn main() {
    let routes = [
        (Get, "/", lang_detect)
    ];

    let server = Server {
        handlers: Router::from_routes(routes),
        port: get_server_port()
    };

    server.run();
}
