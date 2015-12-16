
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

use std::net::{TcpListener, TcpStream};
use std::thread;



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
    flag_port: String
}


fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("Listening on port: {:?}", args.flag_port);
}
