


#![feature(slice_patterns)]

//#![feature(phase)]
//#[allow(unused_parens)]

extern crate rustc_serialize;
extern crate docopt;
extern crate tiny_http;
extern crate url;


mod router;
mod url_parser;
mod http;
mod api;
mod table;
mod context;



//extern crate json_macros;

use std::net::SocketAddr;
use std::str::FromStr;

use tiny_http::{Server};
use docopt::Docopt;


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




fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("Listening on port: {:?}", args.flag_port);

    let addr: SocketAddr = SocketAddr::from_str(&format!("127.0.0.1:{}", args.flag_port)).unwrap();

    let server = Server::http(addr).unwrap();

    let mut tables = table::Tables::new();

    let mut context = context::Context::new(tables);

    loop {
        // blocks until the next request is received
        match server.recv() {
            Ok(rq) => {
                match router::handle_request_and_send_response(&mut context, rq) {
                    Ok(_) => (),
                    Err(err) => println!("Error sending response: {}", err)
                }
            },
            Err(e) => { println!("error: {}", e); break }
        };
    }
}
