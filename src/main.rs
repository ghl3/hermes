
#![feature(slice_patterns)]

extern crate rustc_serialize;
extern crate docopt;
extern crate tiny_http;
extern crate url;
//extern crate collections;

use std::io;
use std::net::SocketAddr;
use std::str::FromStr;

use tiny_http::{Server, Request};

use docopt::Docopt;

mod router;
mod handler;
mod url_parser;
mod http;

const USAGE: &'static str = "
Naval Fate.

Usage:
  rust_redis [--port=<p>]
  naval_fate.py (-h | --help)
  naval_fate.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --port=<p>    Port to listen on [default: 5001]
";


#[derive(Debug, RustcDecodable)]
struct Args {
    flag_port: i32
}


pub fn parse_and_handle_request(mut request: Request) -> Result<(), io::Error> {
    let parsed = router::parse_request(&mut request);
    let response = handler::handle_request(parsed);
    request.respond(response)
}


fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("Listening on port: {:?}", args.flag_port);

    let addr: SocketAddr = SocketAddr::from_str(&format!("127.0.0.1:{}", args.flag_port)).unwrap();

    let server = Server::http(addr).unwrap();

    loop {
        // blocks until the next request is received
        match server.recv() {
            Ok(rq) => {
                match parse_and_handle_request(rq) {
                    Ok(_) => (),
                    Err(err) => println!("Error sending response: {}", err)
                }
            },
            Err(e) => { println!("error: {}", e); break }
        };
    }
}
